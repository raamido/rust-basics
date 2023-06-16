fn main() {
    /*  Tuples  */

    let coordinates = (1456.11, 2587.1);
    println!("{:?} {:?}", coordinates.0, coordinates.1);

    // Tuples support Desctructring 💥
    let (x, y) = coordinates;

    println!("{x}");
    println!("{y}")
}
