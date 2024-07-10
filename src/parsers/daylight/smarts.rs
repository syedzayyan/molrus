use std::collections::{HashMap, VecDeque};

use crate::{core::molecule::Molecule, parsers::{elements::read_symbol, error::Error, scanner::{missing_character, Scanner}}};

use super::{smarts_defs::{
    Expr, ExprType, OpCode, SmartsPattern, TreeNode
}, smarts_utils::{eval_atom_expr, eval_bond_expr, parse_atom_expr, parse_bond_expr}, smiles_utils::read_organic};

impl SmartsPattern {
    pub fn new(smarts_string: String) -> SmartsPattern {
        SmartsPattern {
            nodes: Vec::new(),
            root: 0,
            smarts_string: smarts_string,
            chirality: false,
            recursion : false,
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

    pub fn match_smarts(&self, molecule: &Molecule) -> bool {
        let mut matcher = SmartsMatch::new(&self, molecule);
        matcher.match_smarts()
    }
}

struct SmartsMatch<'a> {
    pattern: &'a SmartsPattern,
    molecule: &'a Molecule,
    atom_mapping: Vec<Option<usize>>,
    bond_mapping: Vec<Option<usize>>,
}

impl<'a> SmartsMatch<'a> {
    fn new(pattern: &'a SmartsPattern, molecule: &'a Molecule) -> Self {
        SmartsMatch {
            pattern,
            molecule,
            atom_mapping: vec![None; pattern.nodes.len()],
            bond_mapping: vec![None; pattern.nodes.len()],
        }
    }

    fn match_smarts(&mut self) -> bool {
        self.match_recursive(0)
    }

    fn match_recursive(&mut self, op_index: usize) -> bool {
        if op_index == self.pattern.nodes.len() {
            return true;
        }

        let node = &self.pattern.nodes[op_index];
        match node.op_code {
            OpCode::SeedAtom => self.seed_atom(op_index),
            OpCode::GrowBond => self.grow_bond(op_index),
            OpCode::CloseRing => self.close_ring(op_index),
            // Other OpCodes
            _ => false,
        }
    }

    fn seed_atom(&mut self, op_index: usize) -> bool {
        for (mol_atom_idx, mol_atom) in self.molecule.atoms.iter().enumerate() {
            if eval_atom_expr(&self.pattern.nodes[op_index].data, mol_atom) {
                self.atom_mapping[op_index] = Some(mol_atom_idx);
                if self.match_recursive(op_index + 1) {
                    return true;
                }
                self.atom_mapping[op_index] = None;
            }
        }
        false
    }

    fn grow_bond(&mut self, op_index: usize) -> bool {
        let node = &self.pattern.nodes[op_index];
        let qry_beg = node.parent;
        let qry_end = op_index;

        if let Some(mol_beg) = self.atom_mapping[qry_beg] {
            for (mol_bond_idx, mol_bond) in self.molecule.bonds.iter().enumerate() {
                if mol_bond.source == mol_beg || mol_bond.dest == mol_beg {
                    let mol_end = if mol_bond.source == mol_beg { mol_bond.dest } else { mol_bond.source };
                    if eval_atom_expr(&self.pattern.nodes[qry_end].data, &self.molecule.atoms[mol_end])
                        && eval_bond_expr(&node.data, mol_bond) {
                        self.atom_mapping[qry_end] = Some(mol_end);
                        self.bond_mapping[op_index] = Some(mol_bond_idx);
                        if self.match_recursive(op_index + 1) {
                            return true;
                        }
                        self.atom_mapping[qry_end] = None;
                        self.bond_mapping[op_index] = None;
                    }
                }
            }
        }
        false
    }

    fn close_ring(&mut self, op_index: usize) -> bool {
        let node = &self.pattern.nodes[op_index];
        let qry_beg = node.parent;
        let qry_end = op_index;

        if let (Some(mol_beg), Some(mol_end)) = (self.atom_mapping[qry_beg], self.atom_mapping[qry_end]) {
            if let Some(mol_bond) = self.molecule.get_bond(mol_beg, mol_end) {
                if eval_bond_expr(&node.data, mol_bond) {
                    self.bond_mapping[op_index] = Some(self.molecule.bonds.iter().position(|b| b == mol_bond).unwrap());
                    if self.match_recursive(op_index + 1) {
                        return true;
                    }
                    self.bond_mapping[op_index] = None;
                }
            }
        }
        false
    }
}