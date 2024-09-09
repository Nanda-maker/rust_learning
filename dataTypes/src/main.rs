fn main() {
    // i8 range = -128 to 127
    // u8 range = 0 to 255

    // let a: u8 =random_number() + 57;
    // let a : u8 = random_number().wrapping_add(57); 
    // let a : u8 = match random_number().checked_add(57){
    //     Some(num) => num,
    //     None => {
    //         println!("cannot add");
    //         return;
    //     }
    // };
    // println!("Value of a is {a}");
    let (a,b):(u8,bool) = random_number().overflowing_add(57);
    println!("Value of a is {a} b is {b}");

    let x:f32 = 5.0/ 2.0;
    println!("x is {x}");

    let c = [1,2,3,4,5,6];
    println!("Value is {}", c[5]);

}

    fn random_number() -> u8{
        200
    }