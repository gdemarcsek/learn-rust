pub fn submodule_function() {
    //println!("I am in a submodule");
    super::x();                         // call a function from the parent module
    //x();                              // this wouldn't work, there is no 'x' in scope
    ::bigger_module::submodule::q();    // the "::" prefix tells Rust to search from the root of the module hierarchy
    q();                                // we also could have just called q like this, since it is in the current module
}

fn q() {

}