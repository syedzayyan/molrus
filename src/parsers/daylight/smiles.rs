use crate::{core::{defs::{Atom, Bond}, molecule::Molecule}, parsers::{error::Error, scanner::{missing_character, Scanner}}};

use super::utils::{read_axial, read_bond, read_bracket, read_organic, read_star};
use std::collections::{HashMap, VecDeque};

fn parse_atom(scanner: &mut Scanner) -> Result<Atom, Error> {
    if let Some(bracket_atom) = read_bracket(scanner)? {
        return Ok(bracket_atom);
    }
    if let Some((element, aromatic)) = read_organic(scanner)? {
        let atom_data = Atom {
            element,
            outgoing_bond: Vec::new(),
            isotope: 0,
            hydrogens: 0,
            aromatic,
            f_charge: 0,
            configuration: None,
            ring: false,
            symmetry_class: 0,
            coords_3d : None
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
    let mut prev_atom: Option<usize> = None;
    let mut ring_closures: HashMap<u8, usize> = HashMap::new();
    let mut branch_points: VecDeque<usize> = VecDeque::new();
    let mut last_aromatic_bond_order = 1;

    while let Some(_) = scanner.peek() {
        match scanner.peek() {
            Some('(') => {
                branch_points.push_front(prev_atom.unwrap());
                scanner.pop();
            }
            Some(')') => {
                prev_atom = branch_points.pop_back();
                scanner.pop();
            }
            Some('.') => {
                prev_atom = None;
                scanner.pop();
            }
            _ => {}
        }

        let mut bond_order = read_bond(&mut scanner);
        let bond_axialness = read_axial(&mut scanner);
        let atom_data = parse_atom(&mut scanner)?;
        let aromatic = atom_data.aromatic;

        let curr_index = molecule.atoms.len();
        molecule.add_atom(atom_data);

        // Handle ring closures
        if let Some(digit) = scanner.peek().and_then(|c| c.to_digit(10)) {
            molecule.atoms[curr_index].ring_reverse();
            scanner.pop();
            let ring_number = digit as u8;

            if let Some(other_atom) = ring_closures.remove(&ring_number) {
                let mut bond_order = bond_order as i8;
                // Check if both atoms are aromatic
                if aromatic && molecule.atoms[other_atom].aromatic {
                    // Use alternating bond orders for aromatic bonds
                    bond_order = last_aromatic_bond_order;
                    last_aromatic_bond_order = if last_aromatic_bond_order == 1 { 2 } else { 1 };
                }
                let bond = Bond {
                    source: other_atom,
                    dest: curr_index,
                    arom: aromatic,
                    ring: true,
                    bond_order: bond_order,
                    axialness : bond_axialness.clone()
                };
                molecule.add_bond(bond);
            } else {
                ring_closures.insert(ring_number, curr_index);
            }
        }

        if let Some(last_atom) = prev_atom {
            if aromatic && molecule.atoms[last_atom].aromatic {
                // Use alternating bond orders for aromatic bonds
                bond_order = last_aromatic_bond_order;
                last_aromatic_bond_order = if last_aromatic_bond_order == 1 { 2 } else { 1 };
            }
            let bond = Bond {
                source: last_atom,
                dest: curr_index,
                arom: aromatic,
                ring: false,
                bond_order: bond_order as i8,
                axialness : bond_axialness
            };
            molecule.add_bond(bond);

            // Update H Count for the last atom
            let last_atom_valency = molecule.atoms[last_atom].element.valence_electrons();
            let number_of_bonds = molecule.bonds.iter().filter(|b| b.source == last_atom || b.dest == last_atom).count();
            let modified_h_count = last_atom_valency as i8 - number_of_bonds as i8;
            molecule.h_count_update(last_atom, modified_h_count.max(0) as u8);
        }

        prev_atom = Some(curr_index);
    }

    Ok(molecule)
}