use std::io;

fn main() {

    loop{

        println!("enter something");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("patronum!");

        println!("your input was: {}", input);
    }
}
