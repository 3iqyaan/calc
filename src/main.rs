mod errors;
mod tokenize;
mod compute;
mod test;

use errors::Result;

use std::io;

fn main() -> Result<()>{
    let mut input = String::new();
    println!("Calc (its slang for calculator) v0.1 \nType help for usage information.");
    while input.trim() != "exit" {
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        if input == "help" {
            println!("Available commands:\n
                help - Show this help message\n
                exit - Exit the application\n
                Type any mathematical expression without any letters to evaluate it.\n
                Mult : *\n
                Div : /\n
                Add : +\n
                Sub : -\n
                Modulus : %\n
                Power : ^\n
            "
            );
            continue;
        }
        if !input.is_empty() && input != "exit" {
            input = input.replace(" ", "");
            let tokens = tokenize::tokenize(&input)?;
            let tokens = compute::to_rpn(tokens);
            let result = compute::eval(tokens)?;
            println!("Result: {}", result);

        }
    }
    Ok(())
}

