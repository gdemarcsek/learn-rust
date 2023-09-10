fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    //let s2 = s1;
    //println!("{}, world!", s1);

    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// this would take ownership
fn calculate_length_take_ownership(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    // note: (s, s.len()) directly would not work
    // becuase s is moved into a tuple, so the s
    // name is no longer valid when the call to s.len()
    // happens
    return (s, length)
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}