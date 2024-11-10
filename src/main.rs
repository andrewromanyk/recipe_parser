use anyhow::Result;
use recipe_parser_andrewromanyk::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

fn main_menu() {
    println!("====== Recipe parse 101 =====");
    println!("Welcome to the main menu!");
    println!("Type 'HELP' for list of commands!");
    println!("=============================");
}

fn help() {
    println!("HELP -- prints out the list of acceptable commands");
    println!("MENU -- go to the main menu");
    println!("PARSE <input_file>  -- parses recipe from the specified file");
    println!("CREDITS -- show credits");
    println!("LEAVE -- exits the program")
}

fn exit() {
    println!("Goodbye!");
}

fn credits() {
    println!("=== CREDITS ===");
    println!(" Main coder - Andrii Romaniuk");
    println!(" Code formater - fmt");
    println!(" Special thanks to: docs.rs")
}

fn expect_command() -> Command {
    print!(">");
    let mut input = String::new();
    if io::stdout().flush().is_err() {
        println!("Failed to flush stdout");
    }
    if io::stdin().read_line(&mut input).is_err() {
        println!("Failed to read stdin")
    }
    Command::new(&input)
}

fn read_file(file_name: String) -> Result<String> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    let mut result = String::new();

    for line in reader.lines() {
        let line = line?;
        result.push_str(&line);
        result.push('\n');
    }

    Ok(result)
}

fn main() -> Result<()> {
    main_menu();
    let mut leave = false;

    while !leave {
        let command = expect_command();

        match &command.command {
            Commands::HELP => help(),
            Commands::MENU => main_menu(),
            Commands::LEAVE => leave = true,
            Commands::CREDITS => credits(),
            Commands::PARSE => {
                let input_file = command.input.unwrap();
                let file_in_res = read_file(input_file);
                if file_in_res.is_err() {
                    println!("Couldn't read input file");
                    continue;
                }
                let recipe = Recipe::new(file_in_res.unwrap().as_str());
                if recipe.is_err() {
                    println!("{:?}", recipe.err());
                } else {
                    println!("Your recipe: {:#?}", recipe);
                }
            }
            Commands::ERROR => {
                println!("Wrong command, please try again!");
            }
        }
    }

    exit();

    Ok(())
}
