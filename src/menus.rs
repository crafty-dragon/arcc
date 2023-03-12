pub mod menus {

    use crate::{games::{*, games::{GameFile, game_search}}, print_error};

    use std::{
        fs::File,
        io::{self, Error, Write}, fmt::format,
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
        if !games::game_search(&game).expect("File did was not created after initialization.") {
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
            print_error(e);
            return;
        }

        let input = input.expect("Error was not caught");

        let found = games::game_search(&input);

        if let Err(e) = found {
            print_error(e);
            return;
        }

        let found = found.expect("Error not caught");

        if found {
            println!("That game is already added.");
            return;
        }

        let res_dir = GameFile::prep_dir(&GameFile::new(&input.trim().to_string()));
        if let Err(_) = res_dir {
            let error_message = "The above error occured, aborting add".red().italic().bold();
            println!("{}", error_message);
            return;
        }

        let game_list_file = File::options().append(true).open("./assets/game_list.txt");
        if let Err(e) = game_list_file{
            print_error(e);
            return;
        }

        let mut game_list_file = game_list_file.expect("Error was not caught.");
        let append_result = game_list_file.write(input.as_str().as_bytes());
        if let Err(e) = append_result {
            print_error(e);
            return;
        }

        
        println!("{}, has been added to your list.", input.trim());

        
    }

    pub(crate) fn main_remove() {
        println!("{}", "Choose a game to remove it, all templates, and all characters".red().italic().bold());
        let input_game = get_input();
        if let Err(e) = input_game{
            print_error(e);
            return;
        }
        let input_game = input_game.expect("Not caught");

        let found = game_search(&input_game);
        if let Err(e) = found{
            print_error(e);
            return;
        }

        if !found.expect("Not caught") {
            println!("{}", "Game is not found for removal.".on_red());
            return;
        }

        let disclaimer_msg = format!("Do you understand that by deleting {}, that all characters and templates will also be deleted? \nIf so, type the following exactly:", input_game.trim()).red().italic(); 
        let typed_string = "I understand that this is permanent.".to_string();
        let typed_statement = typed_string.red().on_bright_white().italic();
        println!("{}", disclaimer_msg);
        println!("{}", typed_statement);

        let input = get_input();
        if let Err(e) = input{
            print_error(e);
            return;
        }

        let input = input.expect("Not caught");

        if !input.trim().eq(&typed_string){
            let abort_message = "Disclaimer not acknowledged. \nAborting delete.".yellow().bold();
            println!("{}", abort_message);
            return;
        }


        

    }

    pub(crate) fn main_edit() {
        todo!()
    }

    
}
