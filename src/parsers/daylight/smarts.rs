use crate::parsers::scanner::Scanner;

use super::smarts_defs::SmartsPattern;

impl SmartsPattern {
    pub fn new (smarts_string: String) -> SmartsPattern {
        SmartsPattern {
            nodes : Vec::new(),
            root : 0,
            smarts_string : smarts_string
        }
    }
    fn build_ast_tree (&self) {
        let scanner = Scanner::new(&self.smarts_string);
        while let Some(_) = scanner.peek() {
            
        }
    }
    fn match_molecule (&self) {

    }
}