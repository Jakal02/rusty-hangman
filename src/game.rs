

pub struct Game {
    secret_word: String,
}


impl Game {
    pub fn new(s_len: u8) -> Game {
        Game {
            secret_word: Self::select_secret_word(s_len),
        }
    }

    pub fn get_secret(&self) -> &String {
        &self.secret_word
    }

    fn select_secret_word(_s_len: u8) -> String {
        "secret".to_string()
    }
}