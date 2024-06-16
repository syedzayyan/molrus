use crate::{core::{atoms::{AtomData, AtomIndex}, bonds::{BondData, BondType}, molecule::Molecule}, parsers::{error::Error, scanner::{missing_character, Scanner}}};
use std::collections::{HashMap, VecDeque};
use super::utils::{read_bond, read_bracket, read_organic, read_star};

fn parse_atom(scanner: &mut Scanner) -> Result<AtomData, Error> {
    if let Some(bracket_atom) = read_bracket(scanner)? {
        return Ok(bracket_atom);
    }

    if let Some((element, aromatic)) = read_organic(scanner)? {
        let atom_data = AtomData {
            element,
            isotope: None,
            hydrogens: 0, // This should be improved later
            aromatic,
            f_charge: 0,
            configuration: None,
            ring: false,
            symmetry_class: 0,
        };
        return Ok(atom_data);
    }

    if let Some(unknown_atom) = read_star(scanner)? {
        return Ok(unknown_atom);
    }
    
    Err(missing_character(scanner))
}

pub fn parse_smiles(smiles: &str) -> Result<Molecule, Error> {
    let mut scanner = Scanner::new(smiles);
    let mut molecule = Molecule::new();
    let mut prev_atom: Option<AtomIndex> = None;
    let mut ring_closures: HashMap<u8, AtomIndex> = HashMap::new();
    let mut branch_points: VecDeque<Option<AtomIndex>> = VecDeque::new();

    while let Some(_) = scanner.peek() {
        match scanner.peek() {
            Some('(') => {
                branch_points.push_front(prev_atom);
                scanner.pop();
            }
            Some(')') => {
                prev_atom = branch_points.pop_back().unwrap_or(None);
                scanner.pop();
            }
            _ => {}
        }
        
        let bond_type = read_bond(&mut scanner);
        let atom_data = parse_atom(&mut scanner)?;
        let curr_index = molecule.add_atom(atom_data);

        // Handle ring closures first
        if let Some(digit) = scanner.peek().and_then(|c| c.to_digit(10)) {
            scanner.pop();
            let ring_number = digit as u8;

            if let Some(other_atom) = ring_closures.remove(&ring_number) {
                let bond_data = if bond_type == BondType::Elided {
                    BondData {
                        bond_type: BondType::Single,
                        ring: true,
                    }
                } else {
                    BondData {
                        bond_type: bond_type.clone(),
                        ring: true,
                    }
                };

                molecule.add_bond(other_atom, curr_index, bond_data);
            } else {
                ring_closures.insert(ring_number, curr_index);
            }
        }

        // Handle normal atom connections
        if let Some(prev) = prev_atom {
            let bond_data = if bond_type == BondType::Elided {
                BondData {
                    bond_type: BondType::Single,
                    ring: false,
                }
            } else {
                BondData {
                    bond_type: bond_type.clone(),
                    ring: false,
                }
            };

            molecule.add_bond(prev, curr_index, bond_data);
        }

        prev_atom = Some(curr_index);
    }
    Ok(molecule)
}