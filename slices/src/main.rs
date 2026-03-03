fn main() {
    let mut s = String::from("Hello world");
    let l = first_word_index(&s);

    println!("Original string: `{s}`");
    println!("Index of the space after the first word: {l}");

    // But if we clear the original string in the future
    // we lose access to this first word, even though we have it's last index
    s.clear();
    println!("Cleared string: `{s}`");
    println!("Index is still the same: {l}");

    // ###################################################### //

    let s = String::from("hello world");
    let hello = &s[0..5];
    let alt_hello = &s[..5];
    let world = &s[6..11];
    let alt_world = &s[6..];
    println!("{hello}, {world}!");
    println!("This is alternative {alt_hello}, {alt_world}!");

    // ###################################################### //

    let s = String::from("hello world"); // let mut s
    let f_word = first_word(&s);
    // s.clear(); // error, cuz s is borrowed in word for reading
    println!("First word in string `{s}` is `{f_word}`");

    // ###################################################### //

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn first_word_index(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
