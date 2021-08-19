//!  Transcode BL602 Rhai Script to uLisp

use rhai::{
    AST,
    ASTNode,
    Position,
};

/// Transcode the compiled Rhai Script to uLisp
pub fn transcode(ast: &AST) {
    //  Walk the nodes in the Rhai Abstract Syntax Tree
    ast.walk(&mut transcode_node);
}

/// Transcode the Rhai AST Node to uLisp
fn transcode_node(nodes: &[ASTNode]) -> bool {
    //  Testing: Stop after a few nodes
    unsafe {
        static mut COUNT: u32 = 0;
        COUNT += 1;
        if COUNT > 10 { return false }
    }    
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

    //  Transcode the Node: Expression or Statement
    match node {
        ASTNode::Stmt(_stmt) => {}
        ASTNode::Expr(_expr) => {}
    }

    /* TOOD: 
    Stmt(
        Var(
            11 @ 4:24,
            "LED_GPIO" @ 4:13,
            (),
            4:9,
        ),
    )
    becomes...
    ( let* 
        (( LED_GPIO 11 ))
        ...
    )
    */    

    /* TODO: 
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
    )
    becomes...
    ( bl_gpio_enable_output 11 0 0 )
    */   

    true
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
Node: Stmt(
    FnCall(
        FnCallExpr {
            namespace: Some(
                gpio,
            ),
            hashes: 216275462078594943,
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
Node: Stmt(
    For(
        FnCall {
            name: "range",
            hash: 3173820496823393639,
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
                        hashes: 15100777281115982861,
                        args: [
                            Variable(LED_GPIO #2) @ 14:17,
                            FnCall {
                                name: "%",
                                hash: 9914832268374762033 (native only),
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
                        hashes: 16962790537791066994,
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
*/