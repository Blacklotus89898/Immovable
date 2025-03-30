mod module1;
mod module2; // Points to `src/module2/mod.rs`
mod terminal_app; // Points to `src/terminalApp/mod.rs`

/// This is the main entry point of the application
fn main() {
    module1::function1();
    module2::submodule::function2();
    terminal_app::app::run_app();
}

