use std::fmt;
use std::collections::HashSet;
use std::io::{stdin, stdout, Read, Write};

pub struct Game {
    secret_word: String,
    previous_guesses: HashSet<char>,
    num_guesses: u8,
}

pub enum GameResult {
    Stumped,
    Guessed,
}

impl Game {
    pub fn new(s_len: u8) -> Game {
        Game {
            secret_word: Self::select_secret_word(s_len),
            previous_guesses: vec![].into_iter().collect(),
            num_guesses: 6,
        }
    }

    pub fn get_secret(&self) -> &String {
        &self.secret_word
    }

    fn select_secret_word(_s_len: u8) -> String {
        "secret".to_string().to_uppercase()
    }

    
    fn first_char_from_str(&self, s: &String) -> char {
        s.as_str().chars().next().unwrap().to_uppercase().next().unwrap()
    }

    /// Returns uppercase 1-character guess if user inputs one upper
    /// or lower case ASCII character (A-Z and a-z).  
    /// Returns GuessError if the input is NOT 1 character excluding
    /// newline or is NOT an ASCII character.
    pub fn one_char_user_input(&mut self) -> Result<char, GuessError> {

        let mut s: String = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");


        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }

        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        let first: char = self.first_char_from_str(&s);

        if s.len() != 1 || !first.is_ascii_alphabetic() || self.previous_guesses.contains(&first){
            return Err(GuessError{input: s});
        }
    
        Ok(first)
    }

    /// repeatedly prompts user to enter valid char.  
    /// Records the guess internally, and returns it for convenience
    pub fn let_user_guess(&mut self) -> char{
        let guess: char = loop {
            print!("Enter your next guess: ");
            match self.one_char_user_input() {
                Ok(c) => break c,
                Err(err) => println!("{}", err),
            }
        };
        self.record_guess(guess);
        guess
    }

    /// Record this guess and decrease number of guesses left 
    /// if `guess` is not in the secret word.
    fn record_guess(&mut self, guess:char) {
        self.previous_guesses.insert(guess);
        self.previous_guesses.insert(guess);
        if !self.secret_word.contains(guess) {
            self.num_guesses -= 1;
        }
        
    }

    pub fn is_over(&self) -> Option<GameResult> {
        if self.num_guesses == 0 {
            return Some(GameResult::Stumped)
        }
        let secret_chars: HashSet<char> = self.secret_word.chars().into_iter().collect();
        if secret_chars.is_subset(&self.previous_guesses) {
            return Some(GameResult::Guessed)
        }
        None
    }

    pub fn start_screen(&self) {
        println!(r" 
==================================================
                  Welcome To...                   
                                                  
  _   _   ___   _   _ _____ ___  ___  ___   _   _ 
 | | | | / _ \ | \ | |  __ \|  \/  | / _ \ | \ | |
 | |_| |/ /_\ \|  \| | |  \/| .  . |/ /_\ \|  \| |
 |  _  ||  _  || . ` | | __ | |\/| ||  _  || . ` |
 | | | || | | || |\  | |_\ \| |  | || | | || |\  |
 \_| |_/\_| |_/\_| \_/\____/\_|  |_/\_| |_/\_| \_/
                                                  

          (Press enter key to continue)            ");
    let _ = stdout().flush();
    stdin().read(&mut [0]).unwrap();
    }

    /// prints current game information
    pub fn print_game_screen(&self) {
        println!(r" 
==================================================
                {}                   
                                                  
                    +---+
                    |   |
                    |   O
                    |  /|\
                    |  / 
                    |
                =========                                              
    {}", self.get_secret_word_progress(), self.get_string_of_previous_guesses());
    }

    /// Get string representing progress on the guess of the secret word
    /// 
    fn get_secret_word_progress(&self) -> String {

        let string: String = self.secret_word.chars().into_iter()
            .map(|x| {
                if self.previous_guesses.contains(&x) {
                    format!("{} ", x.to_uppercase().next().unwrap())
                } else {
                    "_ ".to_string()
                }
            } ).collect();
        string
    }

    fn get_string_of_previous_guesses(&self) -> String {
        self.previous_guesses.iter().map(|x| format!("{} ", x.to_uppercase().next().unwrap())).collect()
    }
}


#[derive(Debug, Clone)]
pub struct GuessError{
    input: String,
}

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a valid character input or has already been guessed.", self.input)
    }
}

