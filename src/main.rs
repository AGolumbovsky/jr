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


// following code is from nazioverflow
// can be modified to call netsh process
// -------------------------------------
// use std::process::Command;

// fn main() {
//     let output = Command::new("echo")
//         .arg("Hello world")
//         .output()
//         .expect("Failed to execute command");

//     assert_eq!(b"Hello world\n", output.stdout.as_slice());
// }