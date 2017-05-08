use std::io;
use std::io::Write;

fn READ(input: String) -> String {
    return input;
}


fn EVAL(input: String) -> String {
    return input;
}

fn PRINT(input: String){
    println!("{}", input);
}

fn rep(input: String) {
    PRINT(EVAL(READ(input)));
}

fn main() {

    while true {
        let mut input = String::new();
        print!("user> ");
        io::stdout().flush();
        io::stdin().read_line(&mut input);
        if input == "" {
            break;
        }
        rep(String::from(input.trim()));
    }
}