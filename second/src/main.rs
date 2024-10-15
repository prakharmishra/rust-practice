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

    // let x: Option<Option<Option<i32>>> = Some(Some(Some(34)));
    let x: Option<Option<Option<i32>>> = Some(None);
    if let Some(Some(y)) = x {
        println!("Valid value: {:?}", y);
    } else if let Some(z) = x {
        println!("Got z instead: {:?}", z);
    }

    // println!("Hello, world! {}", x);
}
