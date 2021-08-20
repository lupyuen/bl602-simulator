//!  Transcode BL602 Rhai Script to uLisp

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

    //  Testing: Stop after a few nodes
    unsafe {
        static mut COUNT: u32 = 0;
        COUNT += 1;
        if COUNT > 10 { return false }
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
            ident.name,
            transcode_expr(expr),
            "TODO_body"  //  TODO
        ),

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
    //  Compose namespace like `bl_gpio_` or ``
    let namespace = match &expr.namespace {
        Some(ns) => format!("bl_{:#?}_", ns),  //  TODO
        None => "".to_string()
    };
    //  Compose arguments
    let args = expr.args.iter().map(|arg| {
        //  Transcode each argument
        transcode_expr(&arg) + " "
    });
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
    format!(
        "( {}{} {})",
        namespace,  //  `bl_gpio_` or ``
        expr.name,  //  `enable_output`
        args.collect::<String>()
    )
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
            hashes: 569519487578098579,
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

            ( bl_gpio_enable_output [
    Variable(LED_GPIO #1) @ 7:29,
    StackSlot(0) @ 7:39,
    StackSlot(1) @ 7:42,
] )
            
Node: Stmt(
    For(
        FnCall {
            name: "range",
            hash: 3135654676930801593,
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
                        hashes: 10615082607919427242,
                        args: [
                            Variable(LED_GPIO #2) @ 14:17,
                            FnCall {
                                name: "%",
                                hash: 3966643234239264188 (native only),
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
                        hashes: 16706793717177321089,
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
Unknown stmt: For(
    FnCall {
        name: "range",
        hash: 3135654676930801593,
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
                    hashes: 10615082607919427242,
                    args: [
                        Variable(LED_GPIO #2) @ 14:17,
                        FnCall {
                            name: "%",
                            hash: 3966643234239264188 (native only),
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
                    hashes: 16706793717177321089,
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
)
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
            hashes: 17067249046263552431 (native only),
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

            ( + [
    Variable(a #2) @ 25:9,
    Variable(b #1) @ 25:13,
] )
*/