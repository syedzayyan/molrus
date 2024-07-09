use std::collections::VecDeque;

use crate::parsers::{error::Error, scanner::{missing_character, Scanner}};

use super::{smarts_defs::{
    OpCode, SmartsPattern, TreeNode
}, smarts_utils::{parse_atom_expr, parse_bond_expr}};

impl SmartsPattern {
    pub fn new(smarts_string: String) -> SmartsPattern {
        SmartsPattern {
            nodes: Vec::new(),
            root: 0,
            smarts_string: smarts_string,
            chirality: false,
        }
    }

    // Add Bond to the AST
    pub fn build_ast(&mut self) -> Result<(), Error> {
        let mut scanner = Scanner::new(&self.smarts_string);
        let mut branch_points: VecDeque<usize> = VecDeque::new();
        let mut prev_node: Option<usize> = None;

        while let Some(_) = scanner.peek() {
            match scanner.peek() {
                Some('.') => return Err(Error::Character(scanner.cursor())),
                Some('-') | Some('=') | Some('#') | Some('$') | Some(':') | Some('~')
                | Some('@') | Some('/') | Some('\\') | Some('!') => {
                    if prev_node.is_none() {
                        missing_character(&mut scanner);
                    }
                    let bexpr = parse_bond_expr(0, &mut scanner)?;
                    let node_index = self.nodes.len();
                    self.nodes.push(TreeNode { 
                        op_code: OpCode::GrowBond, 
                        data: bexpr, 
                        parent: prev_node.unwrap(), 
                        visit: false 
                    });
                    prev_node = Some(node_index);
                },
                Some('(') => {
                    branch_points.push_back(prev_node.unwrap());
                    scanner.pop();
                },
                Some(')') => {
                    prev_node = branch_points.pop_back();
                    scanner.pop();
                },
                Some('[') => {
                    scanner.pop();
                    let atom_expr = parse_atom_expr(&mut scanner, self, &mut prev_node)?;
                    let node_index = self.nodes.len();
                    self.nodes.push(TreeNode { 
                        op_code: OpCode::SeedAtom, 
                        data: atom_expr, 
                        parent: prev_node.unwrap_or(node_index), 
                        visit: false 
                    });
                    prev_node = Some(node_index);
                    if scanner.peek() == Some(&']') {
                        scanner.pop();
                    }
                },
                Some('0'..='9') => {
                    // Handle ring closures
                    // Add ring closure logic
                }
                Some('%') => {
                    scanner.pop(); // Skip '%'
                    // Add complex ring closure logic
                }
                _ => {
                    // Parse implicit bonds and atoms
                    // let atom_element = read_symbol(&mut scanner)?;
                    // let atom = 0; // Need to implement default atom creation
                    // let node_index = self.nodes.len();
                    // self.nodes.push(TreeNode { 
                    //     op_code: OpCode::GrowAtom, 
                    //     data: atom_element, 
                    //     parent: prev_node.unwrap_or(node_index), 
                    //     visit: false 
                    // });
                    // prev_node = Some(node_index);
                }
            }
        }
        Ok(())
    }

    // pub fn match_mol(&self, mol: Molecule) {
    //     for nodes in &self.nodes {
    //         match nodes.op_code {
    //             _ => todo!(),
    //         }
    //     }
    // }
}
