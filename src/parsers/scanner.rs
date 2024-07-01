use super::error::Error;

#[derive(Debug)]
pub struct Scanner {
    cursor: usize,
    characters: Vec<char>
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect()
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }
    pub fn back(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(result) => {
                self.cursor = self.cursor - 1;

                Some(result)
            },
            None => None
        }
    }
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(result) => {
                self.cursor = self.cursor + 1;

                Some(result)
            },
            None => None
        }
    }
}

pub fn missing_character(scanner: &mut Scanner) -> Error {
    if scanner.is_done() {
        Error::EndOfLine
    } else {
        println!("Missing Characters: {}", 
        scanner.characters[scanner.cursor()], 
        );
        Error::Character(scanner.cursor())
    }
}

pub fn defined_error(scanner: &mut Scanner, error_string: &str) -> Error {
    if scanner.is_done() {
        Error::EndOfLine
    } else {
        println!("Error at {} and it probably is because: {}", 
        scanner.characters[scanner.cursor()], error_string
        );
        Error::Character(scanner.cursor())
    }
}