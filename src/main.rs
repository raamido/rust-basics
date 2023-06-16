fn main() {
    /*  Structs  */

    struct Rectangle {
        width: f32,
        height: f32,
    }

    let rect1 = Rectangle {
        width: 11.3,
        height: 24.2,
    };

    println!("{:?}", rect1.width);
    println!("{:?}", rect1.height);
}
