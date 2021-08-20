//!  Transcode BL602 Rhai Script to uLisp

use std::convert::TryInto;
use rhai::{
    AST,
    ASTNode,
    Expr,
    FnCallExpr,
    Position,
    Stmt,
};

/// Transcode the compiled Rhai Script to uLisp
pub fn transcode(ast: &AST) {
    //  Walk the nodes in the Rhai Abstract Syntax Tree
    ast.walk(&mut transcode_node);
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
    match node {
        ASTNode::Stmt(stmt) => { transcode_stmt(stmt); }
        ASTNode::Expr(expr) => { transcode_expr(expr); }
    }

    //  Return true to walk the next node in the tree
    true
}

/// Transcode a Rhai Statement to uLisp
fn transcode_stmt(stmt: &Stmt) {
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
        Stmt::Var(expr, ident, _, _) => println!(
            r#"
            ( let* 
                (( {} {} ))
                {}
            )
            "#,
            ident.name,            //  `LED_GPIO`
            transcode_expr(expr),  //  `11`
            "TODO_body"            //  TODO
        ),

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

            //  Transcode to `dotimes`
            println!(
                r#"
                ( dotimes ({} {})
                    {}
                )
                "#,
                id.name,      //  `i`
                upper_limit,  //  `10`
                "TODO_body"   //  TODO
            );

            //  Transcode the Statement Block
            let body = stmts.statements_mut().iter().map(|stmt| {
                //  Transcode each Statement
                transcode_stmt(stmt);
                ""  //  TODO
            });
            println!("TODO_body: {:#?}", body.collect::<String>());
        }        

        //  Function Call: `gpio::enable_output(LED_GPIO, 0, 0)`
        Stmt::FnCall(expr, _) => println!(
            r#"
            {}
            "#,
            transcode_fncall(expr)
        ),

        _ => println!("Unknown stmt: {:#?}", stmt)
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

        _ => format!("Unknown expr: {:#?}", expr)
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

Node: Stmt(
    Var(
        11 @ 4:24,
        "LED_GPIO" @ 4:13,
        (),
        4:9,
    ),
)

            ( let* 
                (( LED_GPIO 11 ))
                TODO_body
            )
            
Node: Stmt(
    FnCall(
        FnCallExpr {
            namespace: Some(
                gpio,
            ),
            hashes: 14643063589770538716,
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

            ( bl_gpio_enable_output LED_GPIO 0 0 )
            
Node: Stmt(
    For(
        FnCall {
            name: "range",
            hash: 17738280775560091951,
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
                        hashes: 1861784796852141795,
                        args: [
                            Variable(LED_GPIO #2) @ 14:17,
                            FnCall {
                                name: "%",
                                hash: 8534142775551669723 (native only),
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
                        hashes: 13070461326013648071,
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

                ( dotimes (i 10)
                    TODO_body
                )
                

            ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
            

            ( time_delay 1000 )
            
TODO_body: ""
Node: Stmt(
    Var(
        40 @ 23:17,
        "a" @ 23:13,
        (),
        23:9,
    ),
)

            ( let* 
                (( a 40 ))
                TODO_body
            )
            
Node: Stmt(
    Var(
        2 @ 24:17,
        "b" @ 24:13,
        (),
        24:9,
    ),
)

            ( let* 
                (( b 2 ))
                TODO_body
            )
            
Node: Stmt(
    FnCall(
        FnCallExpr {
            namespace: None,
            hashes: 2402555959290588140 (native only),
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

            ( + a b )
                      
*/