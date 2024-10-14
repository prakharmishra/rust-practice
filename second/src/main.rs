fn main() {
    let x: Option<String> = Some(String::from("testing"));
    if let Some(y) = &x {
        println!("Valid value: {}", y);
    } else {
        println!("x is not there");
    }

    let x = None;
    let val = if x.is_some() {
        x.unwrap()
    } else {
        340.0
    };

    println!("{val}");

    // println!("Hello, world! {}", x);
}
