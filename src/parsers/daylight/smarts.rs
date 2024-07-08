use std::collections::VecDeque;

use crate::{
    core::molecule::Molecule,
    parsers::{elements::read_symbol, error::Error, scanner::Scanner},
};

use super::{smarts_defs::{
    BondExpr, BondSpec, NodeData, OpCode, SmartsPattern, TreeNode
}, smarts_utils::parse_explicit_smarts_bonds};

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
    fn create_bond(&mut self, bond_expr: BondExpr, atom1: usize, atom2: usize) {
        // Create the bond node
        self.nodes.push(TreeNode {
            op_code: OpCode::GrowBond,
            data: NodeData::Bond(BondSpec {
                bond_expr,
                src: atom1,
                dest: atom2,
                visit: 0,
                grow: true,
            }),
            parent: atom1,
            visit: false,
        });

        // Update neighbors for both atoms
        if let NodeData::Atom(ref mut atom_spec) = self.nodes[atom1].data {
            atom_spec.nbrs.push(atom2);
        }
        if let NodeData::Atom(ref mut atom_spec) = self.nodes[atom2].data {
            atom_spec.nbrs.push(atom1);
        }
    }
    
    pub fn build_ast(&mut self) -> Result<(), Error> {
        let mut scanner = Scanner::new(&self.smarts_string);
        let mut branch_points: VecDeque<usize> = VecDeque::new();
        let mut prev_node: Option<usize> = None;
        let mut bexpr = None;

        while let Some(_) = scanner.peek() {
            match scanner.peek() {
                Some('.') => return Err(Error::Character(scanner.cursor())),
                Some('-') | Some('=') | Some('#') | Some('$') | Some(':') | Some('~')
                | Some('@') | Some('/') | Some('\\') | Some('!') => {
                    bexpr = parse_explicit_smarts_bonds(&mut scanner, 0);
                }
                Some('(') => {
                    if let Some( _ ) = bexpr {
                        return Err(Error::Character(scanner.cursor()))
                    }
                    branch_points.push_front(prev_node.unwrap());
                    scanner.pop();
                }
                Some (')') => {
                    prev_node = branch_points.pop_back();
                    scanner.pop();
                }
                Some('[') => {

                }
                Some('$') => {
                    let recursive_smarts = collect_recursive_smarts(&mut scanner);
                }
                // Handle Simple Rings
                Some('0'..='9') => {}
                // Handle Complex Rings
                Some('%') => {
                    return Err(Error::Character(scanner.cursor()))
                }
                _ => {
                    // Parse Implicit Bonds and Atoms
                    let atom_element = read_symbol(&mut scanner);
                    match atom_element {
                        Ok(atom_num) => {
                            let atom = 0; // need to implement default atom
                            match &bexpr {
                                Some( bond_expr ) => {self.create_bond(bond_expr.clone(), prev_node.unwrap(), atom + 1)}
                                None => unimplemented!()
                            }
                        },
                        Err(_) => {
                            return Err(Error::Character(scanner.cursor()))
                        }
                    }
                }

            }
        }
        Ok(())
    }
    pub fn match_mol(&self, mol: Molecule) {
        for nodes in &self.nodes {
            match nodes.op_code {
                _ => todo!(),
            }
        }
    }
}

// Utility Functions

fn collect_recursive_smarts(scanner: &mut Scanner) {}

// boolean match(int bndIdx) {
//     if (bndIdx == nQryBonds)
//   RK Implementation
//     return true;
//   // ... qryBeg/End from bond[bndIdx], molBeg/End mapping of these
//   if (mapped(qryBeg) && mapped(qryEnd)) {
//     Bond molBnd = molBeg.getBond(molEnd);
//     if (feasibleBond(qryBnd, mbnd))
//       return match(bndIdx + 1);
//   } else if (mapped(qryBeg)) {
//     for (Bond molBnd : molBeg.bonds()) {
//       molEnd = molBnd.getNbr(molBeg);
//       if (feasibleAtom(qryEnd, molEnd) &&
//           feasibleBond(qryBnd, molBnd)) {
//         add(qryEnd, molEnd);
//         if (match(bndIdx + 1))
//           return true;
//         remove(qryEnd);
//   } }
//   // 3. close ring, O(deg(molBeg))
//   // recurse
//   // 2. grow bond, O(deg(molBeg))
//   beg and end are atoms at either end of a bond qryBeg in a query bond and molBeg in the mol bond
//   // recurse
//   // 1. seed atom, O(n)
//   // recurse
//   } else {
//       for (Atom mAtm : mol.atoms()) {
//         if (feasibleAtom(qryBeg, mAtm)) {
//           add(qryBeg, mAtm);
//           if (match(bndIdx))
//             return true;
//           remove(qryBeg);
//   } }
//   }
//     return false;
//   }
