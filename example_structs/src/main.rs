#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let w = 100;
    // let h = 200;
    // let area = calculate_area(w, h);
    // println!("The area of rectangle is {}", area)
    // let rect = (32, 50);
    // let area = calculate_area(rect);
    // println!("The area of rectangle is {}", area)
    let rect = Rectangle {
        width: 32,
        height: 40,
    };
    let area = dbg!(calculate_area(&rect));

    dbg!(&rect);
    println!("The area of {:#?} is {}", rect, area)
}

// fn calculate_area(width: u32, height: u32) -> u32 {
//     width * height
// }
// tuple type
// fn calculate_area(dimensions: (u32, u32)) -> u32 {
//     let (width, height) = dimensions;
//     width * height
// }
fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
