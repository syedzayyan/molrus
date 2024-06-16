#[derive(Debug,PartialEq)]
pub enum Error {
    EndOfLine,
    Character(usize)
}