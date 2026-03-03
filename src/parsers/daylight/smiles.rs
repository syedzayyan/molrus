use crate::{
    core::{
        defs::{Atom, Bond},
        molecule::Molecule,
    },
    parsers::{elements::read_symbol, error::Error, scanner::Scanner},
};

use super::smiles_utils::{read_axial, read_bond, read_bracket, read_organic, read_star};
use std::collections::{HashMap, VecDeque};

fn parse_atom(scanner: &mut Scanner) -> Result<Atom, Error> {
    if let Some(unknown_atom) = read_star(scanner)? {
        return Ok(unknown_atom);
    }
    if let Some(bracket_atom) = read_bracket(scanner)? {
        return Ok(bracket_atom);
    }
    if let Some(element) = read_organic(scanner)? {
        let atom_data = Atom {
            element,
            outgoing_bond: Vec::new(),
            isotope: 0,
            hydrogens: 0,
            aromatic: true,
            f_charge: 0,
            configuration: None,
            ring: false,
            symmetry_class: 0,
            coords_3d: None,
        };
        return Ok(atom_data);
    } else {
        let element = read_symbol(scanner)?;
        let atom_data = Atom {
            element,
            outgoing_bond: Vec::new(),
            isotope: 0,
            hydrogens: 0,
            aromatic: false,
            f_charge: 0,
            configuration: None,
            ring: false,
            symmetry_class: 0,
            coords_3d: None,
        };
        return Ok(atom_data);
    }
}

/// OpenSMILES-like target valence table (atomic number -> allowed valences).
/// Only organic subset for now; extend as needed.
fn target_valences(z: usize) -> &'static [i8] {
    match z {
        5 => &[3],                // B
        6 => &[4],                // C
        7 => &[3, 5],             // N
        8 => &[2],                // O
        15 => &[3, 5],            // P
        16 => &[2, 4, 6],         // S
        9 | 17 | 35 | 53 => &[1], // F, Cl, Br, I
        _ => &[0],                // no known valence → implicit H = 0
    }
}

/// Sum of bond orders incident on atom `atom_idx`.
fn atom_valence(molecule: &Molecule, atom_idx: usize) -> i32 {
    molecule
        .bonds
        .iter()
        .filter(|b| b.source == atom_idx || b.dest == atom_idx)
        .map(|b| b.bond_order as i32)
        .sum()
}

/// Compute implicit H count from SMILES valence table rule:
/// find smallest target valence ≥ current valence, then H = target - valence.
/// If no target valence ≥ valence, H = 0.
fn compute_implicit_h_count(z: usize, valence: i32) -> usize {
    let targets = target_valences(z);
    let valence = valence.max(0);

    let target = targets
        .iter()
        .filter(|&&t| t >= valence as i8)
        .min()
        .copied()
        .unwrap_or(0);

    if target == 0 {
        0
    } else {
        (target - valence as i8).max(0) as usize
    }
}

pub fn parse_smiles(smiles: &str) -> Result<Molecule, Error> {
    let mut scanner = Scanner::new(smiles);
    let mut molecule = Molecule::new();
    let mut prev_atom: Option<usize> = None;
    let mut ring_closures: HashMap<u8, usize> = HashMap::new();
    let mut branch_points: VecDeque<usize> = VecDeque::new();

    while let Some(_) = scanner.peek() {
        match scanner.peek() {
            Some('(') => {
                branch_points.push_front(prev_atom.unwrap());
                scanner.pop();
                continue;
            }
            Some(')') => {
                prev_atom = branch_points.pop_back();
                scanner.pop();
                continue;
            }
            Some('.') => {
                prev_atom = None;
                scanner.pop();
                continue;
            }
            _ => {}
        }

        let bond_order = read_bond(&mut scanner) as i8;
        let bond_axialness = read_axial(&mut scanner);
        let atom_data = parse_atom(&mut scanner)?;
        let aromatic = atom_data.aromatic;

        let curr_index = molecule.atoms.len();
        molecule.add_atom(atom_data);

        // Handle ring closure (digit after the atom)
        let mut bond = None;
        if let Some(digit_char) = scanner.peek().and_then(|c| c.to_digit(10)) {
            let ring_number = digit_char as u8;
            scanner.pop();

            if let Some(&other_atom) = ring_closures.get(&ring_number) {
                // Flip ring flag on both atoms
                molecule.atoms[curr_index].ring_reverse();
                molecule.atoms[other_atom].ring_reverse();

                let mut actual_bond_order = bond_order;
                if aromatic && molecule.atoms[other_atom].aromatic {
                    // For aromatic bonds, use 1 for now; kekulization can come later.
                    actual_bond_order = 1;
                }

                bond = Some(Bond {
                    source: other_atom,
                    dest: curr_index,
                    arom: aromatic,
                    ring: true,
                    bond_order: actual_bond_order,
                    axialness: bond_axialness.clone(),
                });

                ring_closures.remove(&ring_number);
            } else {
                // First side of the closure
                ring_closures.insert(ring_number, curr_index);
            }
        }

        // Handle connection to previous atom (unless ring closure already made the bond)
        if let Some(last_atom) = prev_atom {
            let mut actual_bond_order = bond_order;
            if aromatic && molecule.atoms[last_atom].aromatic {
                actual_bond_order = 1; // aromatic bond order 1; kekulization later
            }

            bond = Some(Bond {
                source: last_atom,
                dest: curr_index,
                arom: aromatic,
                ring: false,
                bond_order: actual_bond_order,
                axialness: bond_axialness,
            });
        }

        // Record bond and update outgoing_bond
        if let Some(bond) = bond {
            let bond_index = molecule.bonds.len();
            molecule.add_bond(bond);

            molecule.atoms[curr_index].add_to_bond_list(bond_index);
            if let Some(last_atom) = prev_atom {
                molecule.atoms[last_atom].add_to_bond_list(bond_index);
            }
        }

        // Recompute implicit H count for the **previous** atom (curr_index is just added)
        if let Some(last_atom) = prev_atom {
            let current_valence = atom_valence(&molecule, last_atom) as i32;
            let h_count =
                compute_implicit_h_count(molecule.atoms[last_atom].element, current_valence);
            molecule.h_count_update(last_atom, h_count);
        }

        prev_atom = Some(curr_index);
    }

    // Final pass: fix last atom too
    if let Some(last_atom) = prev_atom {
        let valence = atom_valence(&molecule, last_atom) as i32;
        let h_count = compute_implicit_h_count(molecule.atoms[last_atom].element, valence);
        molecule.h_count_update(last_atom, h_count);
    }

    Ok(molecule)
}
