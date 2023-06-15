fn main() {
    /*  Match Statement  */
    let gender = "male";

    match gender {
        "male" => println!("user is M"),
        "female" => println!("user is F"),
        _ => println!("user didn't specifiy his / her gender"),
    }
}
