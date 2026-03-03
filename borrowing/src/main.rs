fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    try_change(&mut s1);
    println!("Updated string: {s1}");

    // let r0 = &s1; // cause error on next line cuz can be only one mutable ref and no immutable ones
    let r1 = &mut s1;
    // let r2 = &mut s1; // cause error cuz only one can borrow for mut
    println!("{r1}");

    // let reference_to_nothing = dangle_ref();
}

/*
fn dangle_ref() -> &String { dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we returned a reference to the String, s
} // s is gone from scope and was dropped, so it's memory goes away!! danger!!!
*/

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn try_change(some_string: &mut String) {
    some_string.push_str(", world!");
}
