use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rand::seq::SliceRandom;

pub struct Dictionary {
    words: Vec<String>,
    pub max_len: usize,
    pub min_len: usize,
}

impl Dictionary {
    pub fn new(filepath: &str) -> Dictionary{
        let words: Vec<String> = get_words_from_file(filepath);
        let (min, max) = Self::min_max_word_lens(&words);
        Dictionary {
            words: words,
            max_len: max,
            min_len: min,
        }
    }

    /// return ref to one random word from the words vector with the given length.
    pub fn rdm_word_with_length(&self, length: usize) -> &String {
        let candidate_words: Vec<&String> = self.words.iter()
            .filter(|x| x.len() == length)
            .collect();

        let mut ran: rand::prelude::ThreadRng = rand::thread_rng();
        candidate_words.choose(&mut ran).unwrap()
    }

    fn min_max_word_lens(words: &Vec<String> ) -> (usize, usize) {
        let lens = words.iter().map(|x| x.len()).collect::<Vec<usize>>();
        let max = lens.iter().max().unwrap();
        let min = lens.iter().min().unwrap();

        (min.clone(), max.clone())
    }

}

fn get_words_from_file(filepath: &str) -> Vec<String>{
    let mut list: Vec<String> = vec![];
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            list.push(line.to_ascii_uppercase())
        }
    } else {
        panic!("can't open file {}, please run again with `rusty-hangman` as current directory.", filepath);
    }
    list
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
