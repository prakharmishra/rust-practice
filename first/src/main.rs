fn main() {
    let hello_world_msg = "Hello World!";
    let name = "Prakhar Mishra";
    let age = 34;
    let mut salary = 1234.56;

    salary = salary / 12.0;

    println!("{} {}", hello_world_msg, get_message(name, age, salary));
}

fn get_message(name: &str, age: i32, salary: f64) -> String {
    format!("My name is {}, age {}. I am earning ${} per month.", name, age, salary)
}