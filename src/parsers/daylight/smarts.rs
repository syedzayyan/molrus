use crate::{core::{defs::{Atom, Bond}, molecule::Molecule}, parsers::{error::Error, scanner::{missing_character, Scanner}}};

use super::utils::{read_bracket, read_organic};

#[derive(Clone, Debug)]
pub enum OpCode {
    SeedAtom,
    GrowBond,
    CloseRing,
    SamePart,
    DiffPart,
    RxnRole,
    TetraLeft,
    TetraRight,
}

#[derive(Clone, Debug)]
pub enum NodeData {
    Atom(AtomExpr),
    Bond(BondExpr),
    None,
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    op_code: OpCode,
    data: NodeData,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct Tree {
    nodes: Vec<TreeNode>,
    root: usize,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            root: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AtomExpr {
    Leaf { atom_type: i32, value: i32 },
    Recursive { atom_type: i32, recur: usize },
    Monadic { atom_type: i32, arg: usize },
    Binary { atom_type: i32, left: usize, right: usize },
}

#[derive(Clone, Debug)]
pub enum BondExpr {
    Monadic { bond_type: i32, arg: usize },
    Binary { bond_type: i32, left: usize, right: usize },
}

pub struct SmartsPatternMatcher {
    tree: Tree,
    smarts_string: String,
}

impl SmartsPatternMatcher {
    pub fn new() -> Self {
        SmartsPatternMatcher {
            tree: Tree::new(),
            smarts_string: String::new(),
        }
    }

    pub fn init(&mut self, pattern: &str) -> bool {
        self.smarts_string = pattern.to_string();
        self.parse_pattern().unwrap_or(false)
    }

    fn parse_pattern(&mut self) -> Result<bool, Error> {
        let mut scanner = Scanner::new(self.smarts_string.as_str());
        let mut current_parent: Option<usize> = None;

        while let Some(_) = scanner.peek() {
            match scanner.peek() {
                Some('(') => {
                    scanner.pop();
                    let branch_point = current_parent.unwrap_or(self.tree.root);
                    current_parent = Some(branch_point);
                }
                Some(')') => {
                    scanner.pop();
                    current_parent = self.tree.nodes[current_parent.unwrap()].parent;
                }
                Some('.') => {
                    scanner.pop();
                    current_parent = None;
                }
                // Some('$') => {
                //     // Handle recursive SMARTS
                //     scanner.pop();
                //     let recursive_smarts = self.read_recursive_smarts(&mut scanner)?;
                //     let recur_index = self.parse_recursive_smarts(&recursive_smarts)?;
                // },
                _ => {
                    let bond_expr = read_smarts_bond(&mut scanner)?;
                    let atom_expr = read_smarts_atom(&mut scanner)?;

                    if current_parent.is_none() {
                        self.push_op_code(OpCode::SeedAtom, NodeData::Atom(atom_expr), None);
                        current_parent = Some(self.tree.nodes.len() - 1);
                    } else {
                        self.push_op_code(OpCode::GrowBond, NodeData::Bond(bond_expr), current_parent);
                        self.push_op_code(OpCode::SeedAtom, NodeData::Atom(atom_expr), Some(self.tree.nodes.len() - 1));
                        current_parent = Some(self.tree.nodes.len() - 1);
                    }

                    // Check for stereochemistry
                    if let Some('@') = scanner.peek() {
                        scanner.pop();
                        if let Some('@') = scanner.peek() {
                            scanner.pop();
                            self.push_op_code(OpCode::TetraRight, NodeData::None, Some(current_parent.unwrap()));
                        } else {
                            self.push_op_code(OpCode::TetraLeft, NodeData::None, Some(current_parent.unwrap()));
                        }
                    }
                }
            }
        }

        Ok(true)
    }

    fn push_op_code(&mut self, op_code: OpCode, data: NodeData, parent: Option<usize>) {
        let node = TreeNode {
            op_code,
            data,
            parent,
            children: Vec::new(),
        };
        let index = self.tree.nodes.len();
        self.tree.nodes.push(node);
        if let Some(parent_index) = parent {
            self.tree.nodes[parent_index].children.push(index);
        } else {
            self.tree.root = index;
        }
    }

    pub fn match_molecule(&self, mol: &Molecule) -> bool {
        let mut atom_mapping = vec![None; self.tree.nodes.len()];
        let mut bond_mapping = vec![None; self.tree.nodes.len()];
        self.match_from_node(self.tree.root, mol, &mut atom_mapping, &mut bond_mapping)
    }

    fn match_from_node(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>, bond_mapping: &mut Vec<Option<usize>>) -> bool {
        let node = &self.tree.nodes[node_index];
        
        match node.op_code {
            OpCode::SeedAtom => self.match_seed_atom(node_index, mol, atom_mapping, bond_mapping),
            OpCode::GrowBond => self.match_grow_bond(node_index, mol, atom_mapping, bond_mapping),
            OpCode::CloseRing => self.match_close_ring(node_index, mol, atom_mapping, bond_mapping),
            OpCode::SamePart => self.match_same_part(node_index, mol, atom_mapping),
            OpCode::DiffPart => self.match_diff_part(node_index, mol, atom_mapping),
            OpCode::RxnRole => self.match_rxn_role(node_index, mol, atom_mapping),
            OpCode::TetraLeft => self.match_tetra_left(node_index, mol, atom_mapping),
            OpCode::TetraRight => self.match_tetra_right(node_index, mol, atom_mapping),
        }
    }

    fn match_seed_atom(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>, bond_mapping: &mut Vec<Option<usize>>) -> bool {
        let node = &self.tree.nodes[node_index];
        if let NodeData::Atom(atom_expr) = &node.data {
            for (m_atom_idx, mol_atom) in mol.atoms.iter().enumerate() {
                if atom_mapping[node_index].is_none() && self.is_atom_feasible(atom_expr, mol_atom) {
                    atom_mapping[node_index] = Some(m_atom_idx);
                    if self.match_children(node_index, mol, atom_mapping, bond_mapping) {
                        return true;
                    }
                    atom_mapping[node_index] = None;
                }
            }
        }
        false
    }

    fn match_grow_bond(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>, bond_mapping: &mut Vec<Option<usize>>) -> bool {
        let node = &self.tree.nodes[node_index];
        if let NodeData::Bond(bond_expr) = &node.data {
            let parent_index = node.parent.unwrap();
            let m_beg = atom_mapping[parent_index].unwrap();
            for (mol_bond_idx, mol_bond) in mol.bonds.iter().enumerate() {
                if mol_bond.source == m_beg || mol_bond.dest == m_beg {
                    let m_end = if mol_bond.source == m_beg { mol_bond.dest } else { mol_bond.source };
                    if atom_mapping[node_index].is_none() && self.is_bond_feasible(bond_expr, mol_bond) {
                        atom_mapping[node_index] = Some(m_end);
                        bond_mapping[node_index] = Some(mol_bond_idx);
                        if self.match_children(node_index, mol, atom_mapping, bond_mapping) {
                            return true;
                        }
                        atom_mapping[node_index] = None;
                        bond_mapping[node_index] = None;
                    }
                }
            }
        }
        false
    }

    fn match_close_ring(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>, bond_mapping: &mut Vec<Option<usize>>) -> bool {
        // Implement close ring matching
        unimplemented!()
    }
    fn match_same_part(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>) -> bool {
        // Implement same part matching
        unimplemented!()
    }

    fn match_diff_part(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>) -> bool {
        // Implement different part matching
        unimplemented!()
    }

    fn match_rxn_role(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>) -> bool {
        // Implement reaction role matching
        unimplemented!()
    }

    fn match_tetra_left(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>) -> bool {
        // Implement tetrahedral left stereochemistry matching
        unimplemented!()
    }

    fn match_tetra_right(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>) -> bool {
        // Implement tetrahedral right stereochemistry matching
        unimplemented!()
    }

    fn match_children(&self, node_index: usize, mol: &Molecule, atom_mapping: &mut Vec<Option<usize>>, bond_mapping: &mut Vec<Option<usize>>) -> bool {
        let node = &self.tree.nodes[node_index];
        for &child_index in &node.children {
            if !self.match_from_node(child_index, mol, atom_mapping, bond_mapping) {
                return false;
            }
        }
        true
    }

    fn is_atom_feasible(&self, query_atom: &AtomExpr, mol_atom: &Atom) -> bool {
        // Implement atom feasibility check
        match query_atom {
            AtomExpr::Leaf { atom_type, value } => {
                (*atom_type == mol_atom.element as i32) &&
                (*value == mol_atom.f_charge as i32)
                // Add more conditions as needed
            },
            // Handle other AtomExpr variants
            _ => unimplemented!("Other atom expressions not yet implemented"),
        }
    }

    fn is_bond_feasible(&self, query_bond: &BondExpr, mol_bond: &Bond) -> bool {
        // Implement bond feasibility check
        match query_bond {
            BondExpr::Monadic { bond_type, .. } => {
                match *bond_type {
                    1 => mol_bond.bond_order == 1,
                    2 => mol_bond.bond_order == 2,
                    3 => mol_bond.bond_order == 3,
                    4 => mol_bond.arom,
                    5 => true, // Any bond
                    6 => mol_bond.ring,
                    0 => mol_bond.bond_order == 1, // Implicit single bond
                    _ => false,
                }
            },
            // Handle other BondExpr variants
            _ => unimplemented!("Other bond expressions not yet implemented"),
        }
    }
}



// SMARTS UTILS

pub fn read_smarts_atom(scanner: &mut Scanner) -> Result<AtomExpr, Error> {
    if let Some(bracket_atom) = read_bracket(scanner)? {
        // Convert bracket_atom to AtomExpr
        Ok(AtomExpr::Leaf {
            atom_type: bracket_atom.element as i32,
            value: bracket_atom.f_charge as i32,
        })
    } else if let Some((element, aromatic)) = read_organic(scanner)? {
        Ok(AtomExpr::Leaf {
            atom_type: element as i32,
            value: if aromatic { 1 } else { 0 },
        })
    } else if let Some('*') = scanner.peek() {
        scanner.pop();
        Ok(AtomExpr::Leaf {
            atom_type: 0, // Wildcard atom
            value: 0,
        })
    } else if let Some('$') = scanner.peek() {
        // Handle recursive SMARTS
        scanner.pop();
        let recursive_smarts = read_parenthesized(scanner)?;
        let recur_index = parse_recursive_smarts(&recursive_smarts)?;
        Ok(AtomExpr::Recursive {
            atom_type: 0,
            recur: recur_index,
        })
    } else if let Some('!') = scanner.peek() {
        // Handle negation
        scanner.pop();
        let arg = Box::new(read_smarts_atom(scanner)?);
        Ok(AtomExpr::Monadic {
            atom_type: 1, // Use 1 for negation
            arg: Box::into_raw(arg) as usize,
        })
    } else if let Some('&') = scanner.peek() {
        // Handle AND operation
        scanner.pop();
        let left = Box::new(read_smarts_atom(scanner)?);
        let right = Box::new(read_smarts_atom(scanner)?);
        Ok(AtomExpr::Binary {
            atom_type: 2, // Use 2 for AND
            left: Box::into_raw(left) as usize,
            right: Box::into_raw(right) as usize,
        })
    } else if let Some(',') = scanner.peek() {
        // Handle OR operation
        scanner.pop();
        let left = Box::new(read_smarts_atom(scanner)?);
        let right = Box::new(read_smarts_atom(scanner)?);
        Ok(AtomExpr::Binary {
            atom_type: 3, // Use 3 for OR
            left: Box::into_raw(left) as usize,
            right: Box::into_raw(right) as usize,
        })
    } else {
        Err(missing_character(scanner))
    }
}

pub fn read_smarts_bond(scanner: &mut Scanner) -> Result<BondExpr, Error> {
    match scanner.peek() {
        Some('-') => {
            scanner.pop();
            Ok(BondExpr::Monadic { bond_type: 1, arg: 0 })
        }
        Some('=') => {
            scanner.pop();
            Ok(BondExpr::Monadic { bond_type: 2, arg: 0 })
        }
        Some('#') => {
            scanner.pop();
            Ok(BondExpr::Monadic { bond_type: 3, arg: 0 })
        }
        Some(':') => {
            scanner.pop();
            Ok(BondExpr::Monadic { bond_type: 4, arg: 0 })
        }
        Some('~') => {
            scanner.pop();
            Ok(BondExpr::Monadic { bond_type: 5, arg: 0 }) // Any bond
        }
        Some('@') => {
            scanner.pop();
            Ok(BondExpr::Monadic { bond_type: 6, arg: 0 }) // Ring bond
        }
        Some('!') => {
            // Handle negation
            scanner.pop();
            let arg = Box::new(read_smarts_bond(scanner)?);
            Ok(BondExpr::Monadic {
                bond_type: 7, // Use 7 for negation
                arg: Box::into_raw(arg) as usize,
            })
        }
        Some('&') => {
            // Handle AND operation
            scanner.pop();
            let left = Box::new(read_smarts_bond(scanner)?);
            let right = Box::new(read_smarts_bond(scanner)?);
            Ok(BondExpr::Binary {
                bond_type: 8, // Use 8 for AND
                left: Box::into_raw(left) as usize,
                right: Box::into_raw(right) as usize,
            })
        }
        Some(',') => {
            // Handle OR operation
            scanner.pop();
            let left = Box::new(read_smarts_bond(scanner)?);
            let right = Box::new(read_smarts_bond(scanner)?);
            Ok(BondExpr::Binary {
                bond_type: 9, // Use 9 for OR
                left: Box::into_raw(left) as usize,
                right: Box::into_raw(right) as usize,
            })
        }
        _ => Ok(BondExpr::Monadic { bond_type: 0, arg: 0 }) // Implicit single bond
    }
}

fn read_parenthesized(scanner: &mut Scanner) -> Result<String, Error> {
    let mut content = String::new();
    let mut depth = 1;

    scanner.pop(); // Consume the opening parenthesis

    while let Some(c) = scanner.pop() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        content.push(*c);
    }

    if depth != 0 {
        Err(missing_character(scanner))
    } else {
        Ok(content)
    }
}

fn parse_recursive_smarts(smarts: &str) -> Result<usize, Error> {
    // This function should parse the recursive SMARTS and return an index
    // For now, we'll just return a dummy value
    Ok(0)
}