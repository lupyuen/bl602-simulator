//! Transcode BL602 Rhai Script to uLisp

use std::convert::TryInto;
use rhai::{
    AST,
    ASTNode,
    Expr,
    FnCallExpr,
    Position,
    Stmt,
};
use crate::scope;

/// Transcode the compiled Rhai Script to uLisp
pub fn transcode(ast: &AST) {
    //  Start the first uLisp Scope
    let scope_index = scope::begin_scope("let* ()");

    //  Walk the nodes in the Rhai Abstract Syntax Tree
    ast.walk(&mut transcode_node);

    //  End the first uLisp Scope and get the uLisp S-Expression for the scope
    let output = scope::end_scope(scope_index);
    println!("Transcoded uLisp:\n{}", output);
}

/// Transcode the Rhai AST Node to uLisp
fn transcode_node(nodes: &[ASTNode]) -> bool {
    //  We take the root node, ignore the subnodes
    let node = &nodes[0];

    //  Get the source code position
    let pos = match node {
        ASTNode::Stmt(stmt) => stmt.position(),
        ASTNode::Expr(expr) => expr.position(),
    };

    //  Skip this node if we've already handled it
    unsafe {
        static mut LAST_POSITION: Position = Position::NONE;
        if LAST_POSITION == pos { return true; }
        LAST_POSITION = pos;
        println!("Node: {:#?}", node);
    }

    //  Transcode the Node: Statement or Expression
    let output = match node {
        ASTNode::Stmt(stmt) => transcode_stmt(stmt),
        ASTNode::Expr(expr) => transcode_expr(expr),
    };

    //  Add the transcoded uLisp S-Expression to the current scope
    scope::add_to_scope(&output);

    //  Return true to walk the next node in the tree
    true
}

/// Transcode a Rhai Statement to uLisp
fn transcode_stmt(stmt: &Stmt) -> String {
    match stmt {
        /* Let or Const Statement: `let LED_GPIO = 11`
            Var(
                11 @ 4:24,
                "LED_GPIO" @ 4:13,
                (),
                4:9,
            ),
            becomes...
            ( let* 
                (( LED_GPIO 11 ))
                ...
            )
        */    
        Stmt::Var(expr, ident, _, _) => {
            //  Begin a new uLisp Scope
            scope::begin_scope(
                format!(
                    "let* (( {} {} ))",    //  `let* (( LED_GPIO 11 ))`
                    ident.name,            //  `LED_GPIO`
                    transcode_expr(expr),  //  `11`
                ).as_str()
            );

            //  Scope will end when the parent scope ends
            "".to_string()
        }

        /* For Statement: `for i in range(0, 10) { ... }`
            For(
                FnCall {
                    name: "range",
                    hash: 7910928861698536248,
                    args: [
                        StackSlot(0) @ 10:24,
                        StackSlot(1) @ 10:27,
                    ],
                    constants: [
                        0,
                        10,
                    ],
                } @ 10:18,
                (
                    "i" @ 10:13,
                    None,
                    Block[ ... ] @ 10:31,
                ),
                10:9,
            )
            becomes...
            ( dotimes (i 10)
                ...
            )
        */
        Stmt::For(expr, id_counter, _) => {
            //  TODO: Support `for` counter
            let id    = &id_counter.0;
            let stmts = &mut id_counter.2.clone();

            //  Get the `for` range, e.g. `[0, 10]`
            let range = get_range(expr);
            let lower_limit = range[0];
            let upper_limit = range[1];
            assert!(lower_limit == 0);  //  TODO: Allow Lower Limit to be non-zero

            //  Begin a new uLisp Scope
            let scope_index = scope::begin_scope(
                format!(
                    "dotimes ({} {})",  //  `dotimes (i 10)`
                    id.name,            //  `i`
                    upper_limit,        //  `10`
                ).as_str()
            );

            //  Transcode the Statement Block
            stmts.statements_mut().iter().for_each(|stmt| {
                //  Transcode each Statement
                let output = transcode_stmt(stmt);

                //  Add the transcoded uLisp S-Expression to the current scope
                scope::add_to_scope(&output);
            });

            //  End the uLisp Scope and add the transcoded uLisp S-Expression to the parent scope
            scope::end_scope(scope_index)
        }        

        //  Function Call: `gpio::enable_output(LED_GPIO, 0, 0)`
        Stmt::FnCall(expr, _) => format!(
            "{}",
            transcode_fncall(expr)
        ),

        _ => panic!("Unknown stmt: {:#?}", stmt)
    }
}

/// Transcode a Rhai Expression to uLisp
fn transcode_expr(expr: &Expr) -> String {
    match expr {
        //  Integers become themselves
        Expr::IntegerConstant(i, _) => format!("{}", i),

        //  Variables become their names
        Expr::Variable(_, _, var) => format!("{}", var.2),

        //  Function Call: `gpio::enable_output(LED_GPIO, 0, 0)`
        Expr::FnCall(expr, _) => transcode_fncall(expr),

        _ => panic!("Unknown expr: {:#?}", expr)
    }
}

/// Transcode a Rhai Function Call to uLisp
fn transcode_fncall(expr: &FnCallExpr) -> String {
    /* Function Call: `gpio::enable_output(LED_GPIO, 0, 0)`
        FnCallExpr {
            namespace: Some(
                gpio,
            ),
            hashes: 4301736447638837139,
            args: [
                Variable(LED_GPIO #1) @ 7:29,
                StackSlot(0) @ 7:39,
                StackSlot(1) @ 7:42,
            ],
            constants: [
                0,
                0,
            ],
            name: "enable_output",
            capture: false,
        }
        becomes...
        ( bl_gpio_enable_output 11 0 0 )
    */   

    //  Compose namespace like `bl_gpio_` or ``
    let namespace = match &expr.namespace {
        Some(ns) => format!("bl_{:#?}_", ns),  //  TODO
        None => "".to_string()
    };

    //  Compose arguments
    let args = expr.args.iter().map(|arg| {
        //  Transcode each argument
        let val = match arg {
            //  Transcode a StackSlot by looking up the constants
            Expr::Stack(i, _) => format!("{}", expr.constants[*i]),

            //  Transcode other expressions
            _ => transcode_expr(&arg)
        };
        val + " "
    });

    //  Transcode to uLisp Function Call:
    //  `( bl_gpio_enable_output 11 0 0 )`
    format!(
        "( {}{} {})",
        namespace,                             //  `bl_gpio_` or ``
        rename_function(&expr.name.as_str()),  //  `enable_output`, `+` or `mod`
        args.collect::<String>()               //  `11 0 0 `
    )
}

/// Rename a Rhai Function or Operator Name to uLisp:
/// `%` becomes `mod`
fn rename_function(name: &str) -> String {
    match name {
        "%" => "mod",
        _   => name
    }.to_string()
}

/// Given a Rhai range expression like `range(0, 10)`
/// return the lower and upper limits: `[0, 10]`
fn get_range(expr: &Expr) -> [i32; 2] {
    match expr {
        /* Range Expression: `range(0, 10)`
            FnCall {
            name: "range",
            hash: 7910928861698536248,
            args: [
                StackSlot(0) @ 10:24,
                StackSlot(1) @ 10:27,
            ],
            constants: [
                0,
                10,
            ],
            }
            becomes...
            [0, 10]
        */
        Expr::FnCall(expr, _) => {
            assert!(expr.name == "range");

            //  Compose arguments
            let args = expr.args.iter().map(|arg| {
                //  Transcode each argument
                match arg {
                    //  Transcode a StackSlot by looking up the constants
                    Expr::Stack(i, _) => expr.constants[*i]
                        .clone()
                        .try_cast::<i32>()
                        .expect("Range arg is not integer"),

                    //  Transcode other expressions
                    _ => panic!("Unknown range arg: {:#?}", arg)
                }                
            });

            //  Return the arguments as an array
            let result: Vec<i32> = args.collect();
            result.try_into()
                .expect("Range should have 2 args")
        }

        _ => panic!("Unknown range: {:#?}", expr)
    }
}

/* Output Log:

begin: let* ()
Node: Stmt(
    Var(
        11 @ 4:24,
        "LED_GPIO" @ 4:13,
        (),
        4:9,
    ),
)
begin: let* (( LED_GPIO 11 ))
Node: Stmt(
    FnCall(
        FnCallExpr {
            namespace: Some(
                gpio,
            ),
            hashes: 5011158642447025924,
            args: [
                Variable(LED_GPIO #1) @ 7:29,
                StackSlot(0) @ 7:39,
                StackSlot(1) @ 7:42,
            ],
            constants: [
                0,
                0,
            ],
            name: "enable_output",
            capture: false,
        },
        7:15,
    ),
)
add:   ( bl_gpio_enable_output LED_GPIO 0 0 )
Node: Stmt(
    For(
        FnCall {
            name: "range",
            hash: 4586966905083840977,
            args: [
                StackSlot(0) @ 10:24,
                StackSlot(1) @ 10:27,
            ],
            constants: [
                0,
                10,
            ],
        } @ 10:18,
        (
            "i" @ 10:13,
            None,
            Block[
                FnCall(
                    FnCallExpr {
                        namespace: Some(
                            gpio,
                        ),
                        hashes: 7431606789252924687,
                        args: [
                            Variable(LED_GPIO #2) @ 14:17,
                            FnCall {
                                name: "%",
                                hash: 2687518567151262708 (native only),
                                args: [
                                    Variable(i #1) @ 15:17,
                                    StackSlot(0) @ 15:21,
                                ],
                                constants: [
                                    2,
                                ],
                            } @ 15:19,
                        ],
                        constants: [],
                        name: "output_set",
                        capture: false,
                    },
                    13:19,
                ),
                FnCall(
                    FnCallExpr {
                        namespace: None,
                        hashes: 832275683387471643,
                        args: [
                            StackSlot(0) @ 19:24,
                        ],
                        constants: [
                            1000,
                        ],
                        name: "time_delay",
                        capture: false,
                    },
                    19:13,
                ),
            ] @ 10:31,
        ),
        10:9,
    ),
)
begin: dotimes (i 10)
add:   ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
add:   ( time_delay 1000 )
add:   ( dotimes (i 10) 
  ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
  ( time_delay 1000 )
)
Node: Stmt(
    Var(
        40 @ 23:17,
        "a" @ 23:13,
        (),
        23:9,
    ),
)
begin: let* (( a 40 ))
Node: Stmt(
    Var(
        2 @ 24:17,
        "b" @ 24:13,
        (),
        24:9,
    ),
)
begin: let* (( b 2 ))
Node: Stmt(
    FnCall(
        FnCallExpr {
            namespace: None,
            hashes: 15218087706094760923 (native only),
            args: [
                Variable(a #2) @ 25:9,
                Variable(b #1) @ 25:13,
            ],
            constants: [],
            name: "+",
            capture: false,
        },
        25:11,
    ),
)
add:   ( + a b )
Transcoded uLisp:
( let* () 
  ( let* (( LED_GPIO 11 )) 
  ( bl_gpio_enable_output LED_GPIO 0 0 )
  ( dotimes (i 10) 
  ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
  ( time_delay 1000 )
)
  ( let* (( a 40 )) 
  ( let* (( b 2 )) 
  ( + a b )
)
)
)
)
                    
*/