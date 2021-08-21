//! Manage uLisp Scope for Transcoder
//! A uLisp Scope begins with a uLisp Header like
//!   `let* (( LED_GPIO 11 ))` or
//!   `dotimes (i 10)`
//! Followed by a list of uLisp S-Expressions like
//!   `( bl_gpio_enable_output LED_GPIO 0 0 )` or
//!   `( bl_gpio_output_set LED_GPIO ( mod i 2 ) )`

/// List of all active uLisp scopes
static mut ALL_SCOPES: Vec<Scope> = Vec::new();

/// uLisp Scope
struct Scope {
    /// Header for the scope like
    ///   `let* (( LED_GPIO 11 ))` or
    ///   `dotimes (i 10)`
    header: String,

    /// List of uLisp S-Expressions for the scope like
    ///   `( bl_gpio_enable_output LED_GPIO 0 0 )` or
    ///   `( bl_gpio_output_set LED_GPIO ( mod i 2 ) )`
body: Vec<String>,
}

/// Create a new scope under the curent scope. Return the new scope index. First scope has index 0.
/// Header looks like
///   `let* (( LED_GPIO 11 ))` or
///   `dotimes (i 10)`
pub fn begin_scope(header: &str) -> usize {
    println!("begin: {}", header);
    0  //  TODO
}

/// End the scope with the index. Return the uLisp S-Expression for the scope. Result looks like
/// ```lisp
/// ( let* (( LED_GPIO 11 ))
///   ( bl_gpio_enable_output LED_GPIO 0 0 )
///   ...
/// )
/// ```
/// or
/// ```lisp
/// ( dotimes (i 10)
///   ( bl_gpio_output_set LED_GPIO ( mod i 2 ) )
///   ...
/// )
/// ```
pub fn end_scope(index: usize) -> String {
    "".to_string()  //  TODO
}

/// Add a uLisp S-Expression to the current scope. `expr` looks like
///   `( bl_gpio_enable_output LED_GPIO 0 0 )` or
///   `( bl_gpio_output_set LED_GPIO ( mod i 2 ) )`
pub fn add_to_scope(expr: &str) {
    if expr == "" { return; }
    println!("add: {}", expr);
}
