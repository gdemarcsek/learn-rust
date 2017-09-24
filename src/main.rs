/*
 * This snippet summerizes the non-trivial things about Rust - it only focuses on core language features,
 * not basic syntax or the details of libraries or standard, higher level collections. This should be a 
 * quick, comprehensive summary of the language easy to understand for people with some 
 * C++(11) and Java(8) background. It is heavily based on "the" book, see: https://doc.rust-lang.org/book/second-edition
 * 
 * This is something I usually do when learning a new language. This little project summerizes the stuff to know about
 * the language while lets me learn by doing. It's mostly comments, but that's on purpose - all examples are explained with
 * some additional comments. 
 * 
 * György Demarcsek, 2017
*/

fn mutability() {
    println!(" - Mutability");
    // Mutability is always explicit in Rust, and by default, everything is immutable
    let x = 42;             // (type is inferred by the compiler whenever possible)
    //x = 21;               // This will fail because x was not declared to be mutable
    let mut x = 42;         // Now 'x' is declared to be mutable, thus...
    x = 21;                 // its value can be changed later on
}

fn shadowing() {
    println!(" - Shadowing");
    let x = "shadowing";
    let x = x.len();            // Shadows the previous declaration
    assert!(x == 9);
}

fn ownership() {
    println!(" - Ownership");
    // -Ownership-
    // Ownerhsip means responsibility of memory management, like in C++. Rules of ownership in Rust:
    // 1. Each value in Rust has a variable that’s called its owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    // The above rules prevent data races, dangling pointers and double free-s at compile time!

    { // scope
        let s = String::from("dynamically allocated string");           // s owns the allocated memory...

        // ...so it will be freed when the scope of s is exited (like dtors in C++, in Rust, this is the drop method)
        // This is because of Rule #3
    }

    // -Copy and Move schemantics-

    // Integeres are simple scalar values of a size known in compile time, so this will do a copy:
    let x = 5;
    let y = x;          // a copy of the value of 'x'
    
    // On the other hand, String-s (not literals) are not copiable (they do not posess the Copy trait, more on that later)
    let s1 = String::from("hello");
    let s2 = s1;        // the inner pointer is copied over and s1 has been invalidated - this is a move operation that transfers ownership
    // This is where Rule #2 is manifested:
    //println!("{}, world!", s1);     // this is now an invalid reference since it would refer to a value that has been moved
    // This also prevents double free-ing the allocated memory: there is a single owner responsible for deallocation

    // One can of course still make a "deep copy" by allocating another memory slice and copying the data into it - the String struct implements the Clone trait.
    // Thus we can write:
    let s3 = s2.clone();

    // The Copy and Move traits are mutually exclusive - a type may not implement both. Copy is used for values of predefined size and Move is used
    // for dyanmically sized data. 

    // -Functions-
    // Passing a value to a function is the same as variable assignment from the ownership point of view:
    // Passing means transferring ownership to the callee:
    takes_ownership(s3);
    //println!("{}", s3);         // s3 has been moved, so this would fail in compile time
    // Returning means passing ownership to the caller:
    let s4 = gives_ownership();
}

fn takes_ownership(s : String) {
    assert!(0 < s.len());
}

fn gives_ownership() -> String {
    let s = String::from("I own this, but not for long...");
    return s;
}

fn references_and_borrowing() {
    println!(" - References and borrowing");
    // The rules of references:
    // 1. At any given time, you can have either but not both of: one mutable reference; any number of immutable references
    // 2. References must always be valid.
    
    // References do not have ownership - passing by reference means passing without transferring ownership
    let s1 = String::from("hello");
    check_length(&s1, 5);

    // References can also be mutable, giving the callee the right to mutate the value
    //append(&mut s1, String::from(" world"));      // this would fail at compile time, because the referenced variable is not mutable
    
    let mut s1 = String::from("hello");
    append(&mut s1, String::from(" world"));        // mutability can be seen as part of the type of the variable
    assert!(s1 == "hello world");

    // There can be multiple references to a varible (it may be borrowed multiple times), but if a mutable reference borrows it,
    // then no other reference can borrow it within the scope of the mutable reference:

    {
        let r1 = &mut s1;
        //let r2 = &s1;             // this would fail, since r1 is a mutable reference to the same variable
        //let r3 = &mut s1;         // smae here...
    }

    let r2 = &s1;                   // but out of the scope of 'r1', we can create a reference again...
    let r3 = &s1;                   // ...even multiple immutable ones

    // This was how Rule #1 is enforced

    // Again, these restrictions are to prevent data races at compile time! Also, the compiler will prevent you from using a dangling reference:
    //let dangling_reference = dangle_around();  // the compiler ensures that the data will not go out of scope before the reference to the data does
    // This was how Rule #2 is enforced
}

fn check_length(s: &String, l: usize) -> bool {
    s.len() == l
    // the value of 's' is bound to another variable in a different scope - no need to free it here, 's' has no ownership of the value
    // the check_length function borrows the value of 's'
}

fn append(s: &mut String, other: String) {
    s.push_str(&other);
}

/*fn dangle_around() -> &String {
    let s = String::from("yolo");
    &s
} // the scope of s ends here, thus it could become invalid, thus returning a reference to it is unsafe - even the definition of this function is a compile time error!
*/


fn slices() {
    // A slice is a reference to a contiguous sequence of elements in a collection
    let mut s = String::from("hello world");
    let slice1 = &s[1..4];
    assert!(slice1 == "ell");
    //let slice2 = &s[0..999];            // Now this cannot be caught be the compiler, since the size of 's' could be determined in run-time, so this will cause a segfault
    let slice2 = &s[..6];
    assert!(slice2 == "hello ");
    // However, this is prevented by the compiler: even though 's' is mutable, it was already borrowed as immutable by 'slice1' and 'slice2', thus
    // there might be some parts of the code that assume the value of 's' could not change:
    //s.clear();

    // Of course, mutable slices can be moved:
    let mut slice3 = &s[1..6];
    slice3 = &s[3..5];
    assert!(slice3 == "lo");

    // Declaring a slice without assignment:
    let slice4: &[i32];         // the type of each item must be 'i32'
}

fn structs() {
    println!(" - Structs");
    #[derive(Debug)]            // easily make our struct printable with the println! macro
    struct User {
        username: String,
        active: bool,
        number_of_friends: u32,
    }

    let u1 = User {
        username: String::from("john"),
        active: true,
        number_of_friends: 123,
    };

    let u2 = User {
        username: String::from("joe"),
        ..u1            // get the rest of the values from 'u1'
    };

    let username = String::from("peter");

    let u3 = User {
        username,       // if there is a variable in scope named the same way as the field, we do not have to repeat ourselves
        ..u1
    };

    assert!(u1.username != u2.username);
    assert!(u2.active);
    assert!(u3.active);

    // Rust also has 'tuple' structs that are just named tuple types of fixed length
    struct RGBColor(i16, i16, i16);
    let red = RGBColor(0xff, 0x00, 0x00);
}


fn methods() {
    println!(" - Methods");
    struct Rectangle {
        width: f32,
        height: f32,
    };

    impl Rectangle {
        fn area(&self) -> f32 { // we do not want to take ownership
            self.height * self.width
        }

        // these are associated functions (often called static functions in other languages)
        fn square(s: f32) -> Rectangle {
            Rectangle { width: s, height: s }
        }

        fn new(width: f32, height: f32) -> Rectangle {
            Rectangle { width, height }
        }
    }

    let r = Rectangle::new(3.4, 4.2);
    let s = Rectangle::square(42.0);

    assert!(s.area() > r.area());
}

fn enums_and_pattern_matching() {
    println!(" - Enums and pattern matching");
    // -Enums-
    // Enums are collections of types (variants) that may also hold various types of data
    enum IpAddress {
        V4(u32),        // you can use any type of data, even structrs here
        V6(u64, u64),
    }

    let home = IpAddress::V4(2130706433);
    let foreign = IpAddress::V6(314159263, 123442);

    // A common pattern: the Option enum
    // An Option represents a value that can be "something" or "nothing"
    // Consequently, an Option has two variants: Some and None - Some can hold any type of data through generics
    let some_ip = Some(home);
    //let absent_ip = None;       // the compiler cannot infer the type of absent_ip here
    let absent_ip: Option<IpAddress> = None; 

    // Now, let's use options:
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y;            // this will be a compile time error: we are trying to add an Option<i8> and i8 which is an undefined operation


    // -The match operator-
    // match allows us to compare a value against a series of patterns and then execute code based on which pattern matches

    enum RPCCallError {
        NetworkError,
        SerilizationError(String),
        OK
    }

    let error = RPCCallError::OK;

    let retcode = match error {     // match forces us to cover all possible cases
        RPCCallError::NetworkError => {
            println!("Network error occurred!");
            100
        },
        RPCCallError::SerilizationError(className) => {
            println!("Failed to serialize class: {}", className);
            200
        }
        RPCCallError::OK => 0,
        _ => -1,    // _ is a placeholder that matches any value - if we put it as last, this will mean what the "default" label menas for "switch" in C++
    };

    // Now if we only want to match for one particular variant and do nothing in other cases, we can use "if let":
    let some_value = Some(0xff);
    if let Some(0xff) = some_value {
        // ^ pattern = expr
        // yay, do something
    } else {
        assert!(false);
    }

    // TODO: Include advanced pattern matching black magic from https://doc.rust-lang.org/book/second-edition/ch18-00-patterns.html
}

// Rules of modules in Rust:
// 1. If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs.
// 2. If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs.
mod my_module;          // this is a module declaration - it tells Rust to look for a module definition in a my_module.rs file
mod bigger_module;      

fn modules() {
    my_module::modules();
    use bigger_module::submodule;       // The "use" statement imports items (modules or functions) and puts them into the current scope
    // The above statement makes "submodule" mean "bigger_module::submodule" in the scope
    // We could have also written: use bigger_module::submodule::module_function letting us use the function directly without prefixing it with its parent modules
    submodule::submodule_function();

    use std::*;         // glob import from Rust's standard library
}

fn generics() {
    println!(" - Generics");
    // TODO
}

fn traits() {
    println!(" - Traits");
    // TODO
}

fn closures() {
    println!(" - Closures");
    // TODO
}

fn main() {
    println!("Core language features:");
    // You can use this as a table of contents
    mutability();
    shadowing();
    ownership();
    references_and_borrowing();
    slices();
    structs();
    methods();
    enums_and_pattern_matching();
    modules();
    generics();
    traits();
}
