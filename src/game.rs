use std::fmt;


pub struct Game {
    secret_word: String,
    previous_guesses: Vec<char>,
    is_over: bool,
    num_guesses: u8,
}


impl Game {
    pub fn new(s_len: u8) -> Game {
        Game {
            secret_word: Self::select_secret_word(s_len),
            previous_guesses: vec![],
            is_over: false,
            num_guesses: 8,
        }
    }

    pub fn get_secret(&self) -> &String {
        &self.secret_word
    }

    fn select_secret_word(_s_len: u8) -> String {
        "secret".to_string()
    }

    
    fn first_char_from_str(&self, s: &String) -> char {
        s.as_str().chars().next().unwrap()
    }

    /// Returns uppercase 1-character guess if user inputs one upper
    /// or lower case ASCII character (A-Z and a-z).  
    /// Returns GuessError if the input is NOT 1 character excluding
    /// newline or is NOT an ASCII character.
    pub fn get_user_guess(&self) -> Result<char, GuessError> {
        use std::io::{stdin,stdout,Write};
        let mut s: String = String::new();
        print!("Guess a character: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");


        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }

        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
    
        if s.len() != 1 || !self.first_char_from_str(&s).is_ascii_alphabetic() {
            return Err(GuessError{input: s});
        }
    
        let first: char = self.first_char_from_str(&s);
    
        Ok(first.to_uppercase().next().unwrap())
    }

}


#[derive(Debug, Clone)]
pub struct GuessError{
    input: String,
}

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a valid character input.", self.input)
    }
}

