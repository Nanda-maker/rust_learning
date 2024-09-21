fn main() {
    // let mut s1 = String::from("hello");

    // let len = calculate_length(&mut s1);

    // println!("The length of '{s1}' is {len}.");
    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;
    
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    // dangle references

    // let reference_to_nothing = dangle();
    // fn dangle() -> &String { // dangle returns a reference to a String

    //     let s = String::from("hello"); // s is a new String
    
    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //   // Danger!
}



// fn calculate_length(s: &mut String) -> usize {
//     s.push_str("hellow string");
//     s.len()
// }

