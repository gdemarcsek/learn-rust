pub mod submodule;          // by default, the submodule declaration is private

// Rules of privacy in Rust:
// 1. If an item is public, it can be accessed through any of its parent modules.
// 2. If an item is private, it can be accessed only by the current module and its child modules.

fn x() -> i32 {
    42
}
