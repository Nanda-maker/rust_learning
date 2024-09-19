fn main() {
    // let mut s = String::from("Hello");
    // s.push_str(" world");
    // println!("s:{s}");
    // let s1 = String::from("hello");
    // let mut s2 = s1.clone(); //expensive
    // s2.push_str("this is s2");
    // println!("{s1}, world!");
    // println!("{s2}, world!");

//     let num = 10;
//     let result = add(num);
//     let name = String::from("Nanda");
//     let s = gives_ownership();
//     let s = takes_and_gives_back(s);
//     takes_ownership(name);
//     println!("num is {num} and result is {result}");
//    //  print!("the value is name is {name}") // owner is out of scope and cleared
//    println!("s = {s}");

let s = String::from("Nanda");
let (s1,len) = calculate_len(s);
println!("the len of {s1} is {len}");


}
// fn gives_ownership() -> String{
//     let s = String::from("This is a string from gives_ownership");
//     s
// }

// fn takes_and_gives_back(s:String) -> String{
//     println!("S in takes_and_gives_back {s}");
//     s
// }

// fn takes_ownership(s:String){
//     println!("Inside ownership {s}");
// }

// fn add(x:i32) -> i32
// {
//     x+10
// }

fn calculate_len(s:String)-> (String,usize){
    let result = s.len();
    (s,result)
}