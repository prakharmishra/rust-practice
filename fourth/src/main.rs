use std::io;

fn main() {
    let mut processed_input = String::new();

    while processed_input != "quit" {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("unable to read line");
        processed_input = input.trim().to_lowercase();

        match processed_input.as_str() {
            "hi" => println!("Hey there!"),
            "how are you" => println!("I am fine, thank you!"),
            "quit" => println!("See you soon!"),
            _ => println!("I didn't recognize that one!"),
        }
    }
}
