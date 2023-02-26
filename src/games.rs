pub mod games{
    use std::{fs::{File, self}, io::{self, BufRead, Error}};

    use colored::Colorize;

    #[derive(Debug)]
    pub struct GameFile{
        path_name: String,
        dir_path: String
    }

    impl GameFile{
        pub(crate) fn new(name: &String) -> Self{
            let path_name = name.replace(" ", "_").replace(".","-");
            let dir_path: String = "./assets/".to_string() + &path_name;

            Self { 
                path_name,
                dir_path
             }
        }

        pub(crate) fn prep_dir(self: &Self) -> Result<(), Error>{
            let game_dir = fs::create_dir(self.dir_path.clone());
            if let Err(e) =  game_dir{
                let error_msg = format!("An error has occured with creating the game directory: \n {}\n", e).red().italic().bold();
                println!("{}", error_msg);
                return Err(e);
            }
            
            let template_dir = 
                fs::create_dir(self.dir_path.clone() + "/templates");
            if let Err(e) = template_dir{
                let error_msg = format!("An error has occured with creating the template directory: \n {}\n", e).red().italic().bold();
                println!("{}", error_msg);
                return Err(e);
            }
                
            let char_dir = 
                fs::create_dir(self.dir_path.clone() + "/characters");
            if let Err(e) = char_dir{
                let error_msg = format!("An error has occured with creating the character directory: \n {}\n", e).red().italic().bold();
                println!("{}", error_msg);
                return Err(e);
            }

            let template_file = File::create(self.dir_path.clone() + "/" + &self.path_name + "_tplts.txt");
            if let Err(e) = template_file {
                let error_msg = format!("An error has occured with creating the template list: \n {}\n", e).red().italic().bold();
                println!("{}", error_msg);
                return Err(e);
            }

            let char_file = File::create(self.dir_path.clone() + "/" + &self.path_name + "_chars.txt");
            if let Err(e) = char_file {
                let error_msg = format!("An error has occured with creating the character list: \n {}\n", e).red().italic().bold();
                println!("{}", error_msg);
                return Err(e);
            }

            Ok(())
        }
    }    

    ///Searches the list of games for a chosen game, if there return Ok(true)
    pub(crate) fn game_search(game: &String) -> Result<bool, Error> {
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

    /// Gathers the list of games if found.
    /// Otherwise, creates an empty file in an assets file that is used to hold the list of games.
    pub(crate) fn gather_games() {
        let game_file = File::open("./assets/game_list.txt");

        // Prompt to send if file is empty
        let no_file_prompt = "No games found, creating list.".purple().bold();

        let empty_file_prompt = "No games found".purple().bold();

        // Checks if game_file is a file, if so, list contents 
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
            // Where the file is created if game_file is an error4
            // Is not checked for error as error means directory exist, or permission denied.
            let _asset_dir = fs::create_dir("./assets");
            let _game_file = File::create("./assets/game_list.txt");
            println!("{}", no_file_prompt);
        }
    }
}
