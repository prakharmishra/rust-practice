fn main() {
    let x: Option<String> = Some(String::from("testing"));
    if let Some(y) = x {
        println!("Valid value: {}", y);
    } else {
        println!("x is not there");
    }

    // println!("Hello, world! {}", x);
}
