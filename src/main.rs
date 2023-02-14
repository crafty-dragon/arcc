mod menus;

use colored::{self, Colorize};
use menus::menus::main_menu;
use std::{
    fs::{self, File},
    io::{self, BufRead},
};

fn main() {
    init();
    loop {
        prompt();
        gather_games();
        let _choice = main_menu();
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
    // Prompts {#4bd,6}
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

/// Gathers the list of games if found.
/// Otherwise, creates an empty file in an assets file that is used to hold the list of games.
fn gather_games() {
    let game_file = File::open("./assets/game_list.txt");

    // Prompt to send if file is empty
    let no_file_prompt = "No games found, creating list.".purple().bold();

    let empty_file_prompt = "No games found".purple().bold();

    // Checks if game_file is a file, if so, list contents {#d21,14}
    if let Ok(file) = game_file {
        let reader = io::BufReader::new(file);

        let mut lines_found = false;
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
                lines_found = true;
            }
        }

        if !lines_found {
            println!("{}", empty_file_prompt);
        }
    } else {
        // Where the file is created if game_file is an error {#971,4}
        // Is not checked for error as error means directory exist, or permission denied.
        let _asset_dir = fs::create_dir("./assets");
        let _game_file = File::create("./assets/game_list.txt");
        println!("{}", no_file_prompt);
    }
}
