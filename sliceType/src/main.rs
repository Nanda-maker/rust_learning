fn main() {
    let s = String::from("Hello World");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("hello: {hello}");
    // println!("world: {world}");
    let res = find_first_word(&s);
    // s.clear();
    println!("For string {s} the Result is {res}");
}

fn find_first_word(input: &String) -> &str {
    let s = input.as_bytes();
    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }
    &input[..]
}
