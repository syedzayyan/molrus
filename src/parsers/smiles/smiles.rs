use super::utils::{read_axial, read_bond, read_bracket, read_organic, read_star};
use crate::{
    core::{
        atoms::{AtomData, AtomIndex},
        bonds::BondData,
        molecule::Molecule,
    },
    parsers::{
        error::Error,
        scanner::{missing_character, Scanner},
    },
};
use std::collections::{HashMap, VecDeque};

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
            Some('.') => {
                prev_atom = None; // Reset prev_atom to handle disconnected fragments
                scanner.pop();
            }
            _ => {}
        }

        let mut atom_data = parse_atom(&mut scanner)?;
        let mut bond_order = read_bond(&mut scanner);
        let bond_axialness = read_axial(&mut scanner);

        if atom_data.aromatic { bond_order = 1.5 };

        let mut curr_index: Option<usize> = None;  
        // Handle ring closures first
        if let Some(digit) = scanner.peek().and_then(|c| c.to_digit(10)) {
            atom_data.ring_reverse();
            let index = molecule.add_atom(atom_data.clone());
            curr_index = Some(index);
            scanner.pop();
            let ring_number = digit as u8;

            if let Some(other_atom) = ring_closures.remove(&ring_number) {
                let bond_data = BondData {
                    bond_order : bond_order,
                    axialness : bond_axialness,
                    ring: true,
                };
                molecule.add_bond(other_atom, curr_index.unwrap(), bond_data);
            } else {
                ring_closures.insert(ring_number, curr_index.unwrap());
            }
        }

        if let Some(curr_index) = curr_index {

        } else {
            curr_index = Some(molecule.add_atom(atom_data));
        }

        // Handle normal atom connections
        if let Some(prev) = prev_atom {
            let bond_data = BondData {
                bond_order: bond_order,
                axialness : bond_axialness,
                ring: false,
            };
            molecule.add_bond(prev, curr_index.unwrap(), bond_data);

            // update h-count
            let prev_atom_valency = molecule.atoms.get(prev).unwrap().atom_data.element.valence_electrons();
            let number_of_bonds = molecule.get_num_bonds(prev);
            let modified_h_count = prev_atom_valency as f32 - number_of_bonds;
            prev_atom = Some(curr_index.unwrap());
            molecule.h_count_update(prev, modified_h_count as u8)
        }
    }
    Ok(molecule)
}