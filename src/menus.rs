pub mod menus {
    use std::{
        fs::File,
        io::{self, BufRead, Error, Write},
    };

    use colored::Colorize;

    pub(crate) enum Choice {
        Add,
        Remove,
        Edit,
        Help,
        Quit,
        Games(String),
    }

    pub(crate) fn get_input() -> Result<String, Error>{
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    pub(crate) fn main_menu() -> Result<Choice, Error> {

        let input = get_input()?;

        match input.trim() {
            "+" => Ok(Choice::Add),
            "-" => Ok(Choice::Remove),
            "~" => Ok(Choice::Edit),
            "?" => Ok(Choice::Help),
            "!" => Ok(Choice::Quit),
            s => Ok(Choice::Games(s.to_string())),
        }
    }

    pub(crate) fn game_menu(game: String) {
        if !game_search(&game).expect("File did was not created after initialization.") {
            let help_start_prompt = "The game you chose has not been added, yet. Giving help"
                .black()
                .on_white()
                .bold();
            println!("{}", help_start_prompt);
            main_help();
            return;
        }
        todo!();
    }

    pub(crate) fn main_help() {
        let add_prompt = "To add a game type +".green().bold();
        let remove_prompt = "To remove a game type -".red().bold();
        let edit_prompt = "To edit a game type ~".purple().bold();
        let help_prompt = "To pull this up again type ?".yellow().bold();
        let quit_prompt = "To quit type !".black().on_red().bold();
        println!("{}", add_prompt);
        println!("{}", remove_prompt);
        println!("{}", edit_prompt);
        println!("{}", help_prompt);
        println!("{}", quit_prompt);
    }

    pub(crate) fn main_add() {
        println!("{}", "Input name of game you wish to add".green().bold());
        let input = get_input();
        if let Err(e) = input {
            let error_message = format!("An error occuried. Details: {}", e).red().bold();
            println!("{}", error_message);
            return;
        }

        let input = input.expect("Error was not caught");

        let found = game_search(&input);

        if let Err(e) = found {
            let error_message = format!("An error occuried. Details: {}", e).red().bold();
            println!("{}", error_message);
            return;
        }

        let found = found.expect("Error not caught");

        if found {
            println!("That game is already added.");
            return;
        }

        let game_file = File::options().append(true).open("./assets/game_list.txt");
        if let Err(e) = game_file{
            let error_message = format!("An error occuried. Details: {}", e).red().bold();
            println!("{}", error_message);
            return;
        }

        let mut game_file = game_file.expect("Error was not caught.");
        let append_result = game_file.write(input.as_str().as_bytes());
        if let Err(e) = append_result {
            let error_message = format!("An error occuried. Details: {}", e).red().bold();
            println!("{}", error_message);
            return;
        }
        
        println!("{}, has been added to your list.", input.trim());

        
    }

    pub(crate) fn main_remove() {
        todo!()
    }

    pub(crate) fn main_edit() {
        todo!()
    }

    fn game_search(game: &String) -> Result<bool, Error> {
        let game_file = File::open("./assets/game_list.txt")?;
        let reader = io::BufReader::new(game_file);
        for lines_read in reader.lines() {
            if let Ok(game_line) = lines_read {
                if game_line.trim().eq(game.trim()) {
                    return Ok(true);
                }
            }
        }
        return Ok(false);
    }
}
