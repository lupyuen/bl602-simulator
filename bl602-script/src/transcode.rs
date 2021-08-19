//!  Transcode BL602 Rhai Script to uLisp

/// Transcode the compiled Rhai Script to uLisp
pub fn transcode(ast: &rhai::AST) {
    ast.walk(&mut transcode_node);
}

/// Transcode the Rhai AST Node to uLisp
fn transcode_node(node: &[rhai::ASTNode]) -> bool {
    //  Testing: Stop after a few nodes
    unsafe {
        static mut COUNT: u32 = 0;
        COUNT += 1;
        if COUNT > 5 { return false }
    }    
    println!("Node: {:#?}", node);
    true
}

/* Output Log:

Node: [
    Stmt(
        Var(
            11 @ 4:24,
            "LED_GPIO" @ 4:13,
            (),
            4:9,
        ),
    ),
    Expr(
        11 @ 4:24,
    ),
]
Node: [
    Stmt(
        FnCall(
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
            },
            7:15,
        ),
    ),
]
Node: [
    Stmt(
        FnCall(
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
            },
            7:15,
        ),
    ),
    Expr(
        Variable(LED_GPIO #1) @ 7:29,
    ),
]
Node: [
    Stmt(
        FnCall(
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
            },
            7:15,
        ),
    ),
    Expr(
        StackSlot(0) @ 7:39,
    ),
]

*/