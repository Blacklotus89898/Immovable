mod module1;
mod module2; // Points to `src/module2/mod.rs`

fn main() {
    module1::function1();
    module2::submodule::function2();
}

