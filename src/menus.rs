pub mod menus {
    use std::{
        fs::File,
        io::{self, BufRead, Error},
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

    pub(crate) fn main_menu() -> Result<Choice, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

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
        if !game_search(game).expect("File did was not created after initialization.") {
            let help_start_prompt = "The game you chose has not been added, yet. Giving help"
                .black()
                .on_white()
                .bold();
            println!("{}", help_start_prompt);
            main_help();
            return;
        }
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
        //println!("{}", add_prompt);
    }

    pub(crate) fn add() {
        todo!()
    }

    pub(crate) fn remove() {
        todo!()
    }

    pub(crate) fn edit() {
        todo!()
    }

    fn game_search(game: String) -> Result<bool, Error> {
        let game_file = File::open("./assets/game_list.txt")?;
        let reader = io::BufReader::new(game_file);
        for lines_read in reader.lines() {
            if let Ok(game_line) = lines_read {
                if game_line == game {
                    return Ok(true);
                }
            }
        }
        return Ok(false);
    }
}
