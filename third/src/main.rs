fn main() {
    test_loop();
    
    for i in (1..=10).rev() {
        println!("Value of i: {i}");
    }

    let x = vec![1, 4, 23, 2, 32];
    for i in x {
        println!("Value of i: {i}");
    }

    test_while();
}

fn test_loop() {
    let mut x = 0;

    loop {
        x += 1;
        println!("Value of x: {x}");

        if x > 5 {
            break;
        }
    }
}

fn test_while() {
    let mut input = String::new();

    while input.trim() != "exit" {
        println!("Please enter 'exit' to stop the program");
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("You entered: {input}");
    }
}