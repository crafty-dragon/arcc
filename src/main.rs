mod menus;
mod games;

use std::fmt::Display;

use colored::{self, Colorize};
use games::games::*;
use menus::menus::*;


fn main() {
    init();
    loop {
        prompt();
        gather_games();
        let choice = main_menu();

        if let Err(e) = choice {
            print_error(e);
            continue;
        }

        let choice = choice.expect("Error was not caught.");

        match choice {
            Choice::Add => main_add(),
            Choice::Remove => main_remove(),
            Choice::Edit => main_edit(),
            Choice::Help => main_help(),
            Choice::Quit => {
                println!("Goodbye!");    
                break
            },
            Choice::Games(game) => game_menu(game),
        }
    }
}

/// Initializes the main menu.
fn init() {
    // println!("Starting...");
    println!(
        "{}",
        "Welcome to the Arbitrary RPG Character Creator"
            .bright_green()
            .bold()
    );

    //gather_games()
}

fn prompt() {
    // Prompts 
    // Uses +, -, ~, and ! as selection commands
    let add_prompt = "Add (+)".green().italic();
    let remove_prompt = "Remove (-)".red().italic();
    let edit_prompt = "Edit (~)".yellow().italic();
    let help_prompt = "Help (?)".bright_blue().italic();
    let quit_prompt = "Quit (!)".black().on_red().italic();

    println!(
        "{}, {}, {}, {}, {}",
        add_prompt, remove_prompt, edit_prompt, help_prompt, quit_prompt
    );
}

pub(crate) fn print_error<T: Display>(e :T){
    let error_message = format!("An error occured. Details: {}", e).red().bold();
    println!("{}", error_message);
}
