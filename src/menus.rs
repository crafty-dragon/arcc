pub mod menus {

    use crate::{games::{*, games::{GameFile, game_search}}, print_error};

    use std::{
        fs::{File, self},
        io::{self, Error, Write, BufRead}, fmt::format,
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

        let disclaimer_msg = 
            format!("Do you understand that by deleting {}, that all characters and templates will also be deleted? \nIf so, type the following exactly:", input_game.trim()).red().italic(); 
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

        let removed_dir = GameFile::new(&input_game.trim().to_string());

        let game_list_file = File::open("./assets/game_list.txt");
        let new_game_list = File::create("./assets/game_list_new.txt");

        if let Err(e) = game_list_file{
            print_error(e);
            let removal = fs::remove_file("./assets/game_list_new.txt");
            if let Err(e) = removal {
                print_error(e);
            }
            return;
        }

        if let Err(e) = new_game_list{
            print_error(e);
            return;
        }

        let game_list_file = game_list_file.expect("Not caught!");
        let mut new_game_list = new_game_list.expect("Not caught");
        let reader_old = io::BufReader::new(game_list_file);

        for lines_compare in reader_old.lines(){
            if let Err(e) = lines_compare{
                print_error(e);
                let removal = fs::remove_file("./assets/game_list_new.txt");
                if let Err(e) = removal {
                    print_error(e);
                }
                continue;
            }
            let line = lines_compare.expect("Not caught!");

            if line.trim().eq(input_game.trim()){
                continue;
            }

            let copy_res = new_game_list.write(line.as_bytes());
            if let Err(e) = copy_res {
                print_error(e);
                let removal = fs::remove_file("./assets/game_list_new.txt");
                if let Err(e) = removal {
                    print_error(e);
                }
            }

        }



        let removal = fs::remove_dir_all(removed_dir.get_dir());

        if let Err(e) = removal {
            print_error(e);
            return;
        }

        let _delete_original = fs::remove_file("./assets/game_list.txt");
        let _rename_new = fs::rename("./assets/game_list_new.txt", "./assets/game_list.txt");

        let deleted_successful_message = format!("{}, has successfully been removed.", 
            input_game.trim()).black().on_bright_white();
        println!("{}", deleted_successful_message);


        

    }

    pub(crate) fn main_edit() {
        todo!()
    }

    
}
