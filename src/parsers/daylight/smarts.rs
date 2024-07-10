use std::collections::{HashMap, VecDeque};

use crate::{core::molecule::Molecule, parsers::{elements::read_symbol, error::Error, scanner::{missing_character, Scanner}}};

use super::{smarts_defs::{
    Expr, ExprType, OpCode, SmartsPattern, TreeNode
}, smarts_utils::{parse_atom_expr, parse_bond_expr}, smiles_utils::read_organic};

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
        let mut ring_closures: HashMap<u8, usize> = HashMap::new();

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
                // Handle ring closures
                Some('0'..='9') => {
                    scanner.pop();
                    let ring_number = scanner.curr_character() as u8;
                    if let Some(ring_start_atom) = ring_closures.remove(&ring_number) {
                        let bexpr = Expr {
                            expr_type : ExprType::BeAny,
                            val : None,
                            left : None,
                            right : None,
                        };
                        self.nodes.push(TreeNode {
                            op_code : OpCode ::CloseRing,
                            data : bexpr, 
                            parent : ring_start_atom,
                            visit : false
                        });
                    } else {
                        ring_closures.insert(ring_number, prev_node.unwrap());
                    }
                }
                // Add complex ring closure logic
                Some('%') => {
                    if prev_node.is_none() {
                        missing_character(&mut scanner);
                    }
                    scanner.pop();
                    let mut str_digit = String::from("");
                    match scanner.peek() {
                        Some('0'..='9') => {
                            scanner.pop();
                            str_digit.push(scanner.curr_character());
                            match scanner.peek() {
                                Some('0'..='9') => {
                                    scanner.pop();
                                    str_digit.push(scanner.curr_character());
                                }
                                _ => {}
                            }
                        }
                        _ => return Err(Error::Character(scanner.cursor())),
                    }
                    let ring_number = str_digit.parse::<u8>().unwrap();
                    if let Some(ring_start_atom) = ring_closures.remove(&ring_number) {
                        let bexpr = Expr {
                            expr_type : ExprType::BeAny,
                            val : None,
                            left : None,
                            right : None,
                        };
                        self.nodes.push(TreeNode {
                            op_code : OpCode ::CloseRing,
                            data : bexpr, 
                            parent : ring_start_atom,
                            visit : false
                        });
                    } else {
                        ring_closures.insert(ring_number, prev_node.unwrap());
                    }
                }
                _ => {
                    // Parse implicit atoms
                    let atom_element = read_symbol(&mut scanner);
                    let atom_element_num_resulted: Option<i8>;
                    let arom:bool;
                    match atom_element {
                        Ok( resulted_okay_atom) => {
                            atom_element_num_resulted = Some(resulted_okay_atom as i8);
                            arom = false
                        },
                        Err(_) => {
                            let aromatic_ele = read_organic(&mut scanner)?;
                            arom = true;
                            atom_element_num_resulted = aromatic_ele.and_then(|x| x.try_into().ok());
                        }
                    }
                    let aexpr = Expr {
                        expr_type : if arom {ExprType::AeAromatic} else {ExprType::AeAliphatic},
                        val : atom_element_num_resulted,
                        left : None,
                        right : None,
                    };
                    let node_index = self.nodes.len();
                    self.nodes.push(TreeNode { 
                        op_code: OpCode::SeedAtom, 
                        data: aexpr, 
                        parent: prev_node.unwrap_or(node_index), 
                        visit: false 
                    });
                    let bexpr = Expr {
                        expr_type : if arom {ExprType::BeArom} else {ExprType::BeSingle},
                        val : atom_element_num_resulted,
                        left : None,
                        right : None,
                    };
                    self.nodes.push(TreeNode { 
                        op_code: OpCode::GrowBond, 
                        data: bexpr, 
                        parent: prev_node.unwrap_or(node_index), 
                        visit: false 
                    });
                    prev_node = Some(node_index);
                }
            }
        }
        Ok(())
    }

    pub fn match_mol(&self, mol: Molecule) {
        println!("{:?}", mol.atoms);
        for nodes in &self.nodes {
            match nodes.op_code {
                _ => todo!(),
            }
        }
    }
}
