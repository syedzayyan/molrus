use core::panic;
use std::collections::{HashMap, VecDeque};

use crate::{core::molecule::Molecule, parsers::{elements::read_symbol, error::Error, scanner::{missing_character, Scanner}}};

use super::{smarts_defs::{
    Expr, ExprType, OpCode, SmartsPattern, TreeNode
}, smarts_utils::{collect_recursive_smarts, eval_atom_expr, eval_bond_expr, parse_primitive_bond_types}, smiles_utils::read_organic};

impl SmartsPattern {
    pub fn new(smarts_string: &str) -> SmartsPattern {
        let mut pat = SmartsPattern {
            nodes: Vec::new(),
            root: 0,
            smarts_string: smarts_string.to_string(),
            chirality: false,
            recursion : false,
        };
        let ast_res = pat.build_ast();
        match ast_res {
            Ok(_) => {},
            Err(e) => panic!("{:?}", e)
        }
        pat
    }
    
    // Parse Bracketed Atom Exprs
    fn parse_atom_expr(
        &mut self,
        scanner: &mut Scanner,
        prev_node: &mut Option<usize>,
    ) -> Result<Expr, Error> {
        let mut atomic_num: Option<i8> = None;
        let mut expr_type = ExprType::True;
        match scanner.peek() {
            Some('#') => {
                match scanner.peek() {
                    Some('0'..='9') => {
                        atomic_num = Some(scanner.peek().unwrap().to_owned() as i8);
                    }
                    _ => return Err(missing_character(scanner)),
                }
                scanner.pop();
            }
            Some('$') => {
                let recursive_smarts = collect_recursive_smarts(scanner);
                let mut recursive_smart_instance = SmartsPattern::new(recursive_smarts.as_str());
                recursive_smart_instance.build_ast()?;
                let mut recur_nodes = recursive_smart_instance.nodes;
                recur_nodes[0].data.expr_type = ExprType::AeRecur;
                self.nodes.extend(recur_nodes);
                *prev_node = Some(self.nodes.len());
            }
            Some('+') => {
                expr_type = ExprType::AeCharge;
                scanner.pop();
                match scanner.peek() {
                    Some('0'..='9') => {
                        atomic_num = Some(scanner.peek().unwrap().to_owned() as i8);
                    }
                    _ => return Err(missing_character(scanner)),
                }
                scanner.pop();
            }
            Some('-') => {
                expr_type = ExprType::AeCharge;
                scanner.pop();
                match scanner.peek() {
                    Some('0'..='9') => {
                        atomic_num = Some((scanner.peek().unwrap().to_owned() as i8) * -1);
                    }
                    _ => return Err(missing_character(scanner)),
                }
                scanner.pop();
            }
            Some('@') => {
                scanner.pop();
                match scanner.peek() {
                    Some('?') => {
                        expr_type = ExprType::AlUnspecified;
                    }
                    Some('@') => {
                        expr_type = ExprType::AlAnticlockwise;
                    }
                    _ => expr_type = ExprType::AlClockwise,
                }
                scanner.pop();
            }
            Some('^') => {
                scanner.pop();
                match scanner.peek() {
                    Some('0'..='9') => {
                        scanner.pop();
                        atomic_num = Some(scanner.curr_character() as i8);
                        expr_type = ExprType::AeHyb;
                    }
                    _ => {
                        expr_type = ExprType::AeHyb;
                        atomic_num = Some(1);
                    }
                }
            }
            Some('D') | Some('H') | Some('R') | Some('h') | Some('r') | Some('v') | Some('X')
            | Some('x') => {
                scanner.pop();
                match scanner.peek() {
                    Some('0'..='9') => { 
                        atomic_num = Some(scanner.curr_character() as i8);
                        scanner.pop();
                        match scanner.look_back() {
                            Some('D') => expr_type = ExprType::AeDegree,
                            Some('H') => expr_type = ExprType::AeHcount,
                            Some('h') => expr_type = ExprType::AeImplicit,
                            Some('R') => expr_type = ExprType::AeRings,
                            Some('r') => expr_type = ExprType::AeSize,
                            Some('v') => expr_type = ExprType::AeValence,
                            Some('x') => expr_type = ExprType::AeConnect,
                            Some('X') => expr_type = ExprType::AeRingconnect,
                            _ => {}
                        }
                    }
                    _ => return Err(missing_character(scanner)),
                }
            }
            Some('0'..='9') => {
                let mut str_digit = String::from("");
                str_digit.push(scanner.curr_character());
                scanner.pop();
                match scanner.peek() {
                    Some('0'..='9') => {
                        str_digit.push(scanner.curr_character());
                        scanner.pop();
    
                        match scanner.peek() {
                            Some('0'..='9') => {
                                str_digit.push(scanner.curr_character());
                                scanner.pop();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                let parsed_string = str_digit.parse::<u8>().unwrap();
                if parsed_string == 0 || parsed_string > 118 {
                    panic!("Do better")
                }
                atomic_num = Some(parsed_string as i8);
                scanner.pop();
            }
            Some('*') => {
                expr_type = ExprType::True;
                scanner.pop();
            }
            _ => {
                let atom_element = read_symbol(scanner);
                match atom_element {
                    Ok(resulted_okay_atom) => {
                        atomic_num = Some(resulted_okay_atom as i8);
                        expr_type = ExprType::AeAliphatic;
                    }
                    Err(_) => {
                        let aromatic_ele = read_organic(scanner);
                        match aromatic_ele {
                            Ok(aromm) => {
                                atomic_num = aromm.and_then(|x| x.try_into().ok());
                                expr_type = ExprType::AeAromatic;
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
        }
        Ok(Expr {
            expr_type: expr_type,
            val: atomic_num,
            left: None,
            right: None,
        })
    }
    
    fn parse_atom_expr_in_bracket(&mut self, level: u8, scanner: &mut Scanner, prev_node: &mut Option<usize>) -> Option<Expr> {
        match level {
            // Low-precedence conjunction
            0 => {
                let mut expr = self.parse_atom_expr_in_bracket(1, scanner, prev_node)?;
                while let Some(';') = scanner.peek() {
                    scanner.pop();
                    let right = self.parse_atom_expr_in_bracket(1, scanner, prev_node)?;
                    expr = Expr {
                        expr_type: ExprType::AeAndlo,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: Some(Box::new(right)),
                    };
                }
                Some(expr)
            },
            // Disjunction
            1 => {
                let mut expr = self.parse_atom_expr_in_bracket(2, scanner, prev_node)?;
                while let Some(',') = scanner.peek() {
                    scanner.pop();
                    let right = self.parse_atom_expr_in_bracket(2, scanner, prev_node)?;
                    expr = Expr {
                        expr_type: ExprType::AeOr,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: Some(Box::new(right)),
                    };
                }
                Some(expr)
            },
            // High-precedence conjunction
            2 => {
                let mut expr = self.parse_atom_expr_in_bracket(3, scanner, prev_node)?;
                while let Some('&') = scanner.peek() {
                    scanner.pop();
                    let right = self.parse_atom_expr_in_bracket(3, scanner, prev_node)?;
                    expr = Expr {
                        expr_type: ExprType::AeAndhi,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: Some(Box::new(right)),
                    };
                }
                Some(expr)
            },
            // Negation and primitive expressions
            3 => {
                if let Some('!') = scanner.peek() {
                    scanner.pop();
                    let expr = self.parse_atom_expr_in_bracket(3, scanner, prev_node)?;
                    Some(Expr {
                        expr_type: ExprType::AeNot,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: None,
                    })
                } else {
                    match self.parse_atom_expr(scanner, prev_node) {
                        Ok (good_atom_expr) => Some(good_atom_expr),
                        Err(_) => None,
                    }
                }
            },
            _ => None,
        }
    }
    
    
    
    // Parse Bracketed Bonds
    fn parse_bond_expr(&mut self, level: u8, scanner: &mut Scanner) -> Option<Expr> {
        match level {
            // Low-precedence conjunction
            0 => {
                let mut expr = self.parse_bond_expr(1, scanner)?;
                while let Some(';') = scanner.peek() {
                    scanner.pop();
                    let right = self.parse_bond_expr(1, scanner)?;
                    expr = Expr {
                        expr_type: ExprType::BeAndlo,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: Some(Box::new(right)),
                    };
                }
                Some(expr)
            },
            // Disjunction
            1 => {
                let mut expr = self.parse_bond_expr(2, scanner)?;
                while let Some(',') = scanner.peek() {
                    scanner.pop();
                    let right = self.parse_bond_expr(2, scanner)?;
                    expr = Expr {
                        expr_type: ExprType::BeOr,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: Some(Box::new(right)),
                    };
                }
                Some(expr)
            },
            // High-precedence conjunction
            2 => {
                let mut expr = self.parse_bond_expr(3, scanner)?;
                while let Some('&') = scanner.peek() {
                    scanner.pop();
                    let right = self.parse_bond_expr(3, scanner)?;
                    expr = Expr {
                        expr_type: ExprType::BeAndhi,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: Some(Box::new(right)),
                    };
                }
                Some(expr)
            },
            // Negation and primitive expressions
            3 => {
                if let Some('!') = scanner.peek() {
                    scanner.pop();
                    let expr = self.parse_bond_expr(3, scanner)?;
                    Some(Expr {
                        expr_type: ExprType::BeNot,
                        val: None,
                        left: Some(Box::new(expr)),
                        right: None,
                    })
                } else {
                    Some(parse_primitive_bond_types(scanner))
                }
            },
            _ => None,
        }
    }
    

    
    pub fn build_ast(&mut self) -> Result<(), Error> {
        let mut scanner = Scanner::new(&self.smarts_string);
        let mut branch_points: VecDeque<usize> = VecDeque::new();
        let mut prev_node: Option<usize> = None;
        let mut ring_closures: HashMap<u8, usize> = HashMap::new();
        while let Some(_) = scanner.peek() {
            println!("{:?}", scanner.curr_character());
            match scanner.peek() {
                Some('.') => return Err(Error::Character(scanner.cursor())),
                Some('-') | Some('=') | Some('#') | Some('$') | Some(':') | Some('~')
                | Some('@') | Some('/') | Some('\\') | Some('!') => {
                    if prev_node.is_none() {
                        missing_character(&mut scanner);
                    }
                    let bexpr = self.parse_bond_expr(0, &mut scanner).unwrap();
                    let node_index = self.nodes.len();
                    let src_atom_index = prev_node.unwrap();
                    let dst_atom_index = node_index + 1; // The next node will be the destination atom
    
                    self.nodes.push(TreeNode { 
                        op_code: OpCode::GrowBond, 
                        data: bexpr, 
                        src: src_atom_index, 
                        dst: Some(dst_atom_index),
                        nbrs: None,
                        visit: false 
                    });
    
                    // Update the source atom's nbrs
                    if let Some(nbrs) = &mut self.nodes[src_atom_index].nbrs {
                        nbrs.push(node_index);
                    }
    
                    prev_node = Some(node_index);
                },
                Some('(') => {
                    branch_points.push_back(prev_node.unwrap());
                    scanner.pop();
                },
                Some(')') => {
                    prev_node = branch_points.pop_back();
                    scanner.pop();
                    // If there's a bond after the branch, update the branch point atom's nbrs
                    if scanner.peek().map_or(false, |c| "=-#$:~@/\\!".contains(*c)) {
                        let branch_point = prev_node.unwrap();
                        let bond_index = self.nodes.len(); // The next node will be a bond
                        if let Some(nbrs) = &mut self.nodes[branch_point].nbrs {
                            nbrs.push(bond_index);
                        }
                    }
                },
                Some('0'..='9') => {
                    scanner.pop();
                    let ring_number = scanner.curr_character() as u8;
                    if let Some(ring_start_atom) = ring_closures.remove(&ring_number) {
                        let bexpr = Expr {
                            expr_type: ExprType::BeAny,
                            val: None,
                            left: None,
                            right: None,
                        };
                        let ring_closure_index = self.nodes.len();
                        self.nodes.push(TreeNode {
                            op_code: OpCode::CloseRing,
                            data: bexpr, 
                            src: ring_start_atom,
                            dst: prev_node,
                            nbrs: None,
                            visit: false
                        });
                        // Update nbrs for both atoms involved in the ring closure
                        if let Some(nbrs) = &mut self.nodes[ring_start_atom].nbrs {
                            nbrs.push(ring_closure_index);
                        }
                        if let Some(prev) = prev_node {
                            if let Some(nbrs) = &mut self.nodes[prev].nbrs {
                                nbrs.push(ring_closure_index);
                            }
                        }
                    } else {
                        ring_closures.insert(ring_number, prev_node.unwrap());
                    }
                }
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
                            expr_type: ExprType::BeAny,
                            val: None,
                            left: None,
                            right: None,
                        };
                        let ring_closure_index = self.nodes.len();
                        self.nodes.push(TreeNode {
                            op_code: OpCode::CloseRing,
                            data: bexpr, 
                            src: ring_start_atom,
                            dst: prev_node,
                            nbrs: None,
                            visit: false
                        });
                        // Update nbrs for both atoms involved in the ring closure
                        if let Some(nbrs) = &mut self.nodes[ring_start_atom].nbrs {
                            nbrs.push(ring_closure_index);
                        }
                        if let Some(prev) = prev_node {
                            if let Some(nbrs) = &mut self.nodes[prev].nbrs {
                                nbrs.push(ring_closure_index);
                            }
                        }
                    } else {
                        ring_closures.insert(ring_number, prev_node.unwrap());
                    }
                }
                Some('[') => {
                    scanner.pop();
                    if let Some(atomexpr) = self.parse_atom_expr_in_bracket(0, &mut scanner, &mut prev_node) {
                        println!("{:?}", atomexpr);
                        let node_index = self.nodes.len();
                        let bexpr = if atomexpr.expr_type == ExprType::AeAromatic {ExprType::BeArom} else {ExprType::BeSingle};
                        self.nodes.push(TreeNode { 
                            op_code: OpCode::SeedAtom, 
                            data: atomexpr, 
                            src: prev_node.unwrap_or(node_index), 
                            dst: None,
                            nbrs: Some(vec![]),
                            visit: false 
                        });
                        prev_node = Some(node_index);
                        self.nodes.push(TreeNode { 
                            op_code: OpCode::GrowBond, 
                            data: Expr { 
                                expr_type: bexpr, 
                                val: None, 
                                left: None, 
                                right: None 
                            }, 
                            src: node_index + 1, 
                            dst: None, 
                            nbrs: None, 
                            visit: false 
                        });
                    }
                },
                Some(']') => {
                    scanner.pop();
                },
                _ => {
                    // Parse implicit atoms
                    let atom_expr = self.parse_atom_expr(&mut scanner, &mut prev_node);
                    let node_index = self.nodes.len();
                    self.nodes.push(TreeNode { 
                        op_code: if prev_node.is_some() {OpCode::SamePart} else {OpCode::SeedAtom}, 
                        data: atom_expr?, 
                        src: node_index, 
                        dst: None, 
                        nbrs: Some(Vec::new()), 
                        visit: false 
                    });
                    if scanner.is_done() { break }
                    let bexpr = Expr {
                        expr_type: ExprType::BeSingle ,
                        val: None,
                        left: None,
                        right: None,
                    };
                    let bond_index = self.nodes.len();
                    self.nodes.push(TreeNode { 
                        op_code: OpCode::GrowBond, 
                        data: bexpr, 
                        src: prev_node.unwrap_or(node_index), 
                        dst: Some(node_index),
                        nbrs: None,
                        visit: false 
                    });
                    // Update the previous atom's nbrs
                    if let Some(prev) = prev_node {
                        if let Some(nbrs) = &mut self.nodes[prev].nbrs {
                            nbrs.push(bond_index);
                        }
                    }
                    // Update the current atom's nbrs
                    if let Some(nbrs) = &mut self.nodes[node_index].nbrs {
                        nbrs.push(bond_index);
                    }
                    prev_node = Some(node_index);
                }
            }
            if scanner.is_done() { break }
        }
        Ok(())
    }
    pub fn match_mol(&self, molecule: &Molecule) -> bool {
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
        let qry_beg = node.src;
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
        let qry_beg = node.src;
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