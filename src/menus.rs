pub mod menus {
    use std::io::{self, Error};

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
        let _choice = io::stdin().read_line(&mut input)?;

        match input.as_str() {
            "+" => Ok(Choice::Add),
            "-" => Ok(Choice::Remove),
            "~" => Ok(Choice::Edit),
            "?" => Ok(Choice::Help),
            "!" => Ok(Choice::Quit),
            s => Ok(Choice::Games(s.to_string())),
        }
    }
}
