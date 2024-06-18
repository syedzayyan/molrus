use std::{cmp::Ordering, collections::HashSet};

use crate::core::{atoms::{Atom, AtomIndex}, bonds::{BondData, BondIndex}, molecule::Molecule};
use super::hash::djb2_hash;

// https://depth-first.com/articles/2019/01/11/extended-connectivity-fingerprints/

fn compute_atom_hashes(atom: &Atom) -> u64 {
    let curr_atom_data = atom.atom_data.clone(); 

    // details of current atom
    let atomic_number: i32 = curr_atom_data.element.atomic_number();
    let h_count: i32 = curr_atom_data.hydrogens as i32;
    let f_charge: i32 = curr_atom_data.f_charge.abs() as i32;
    let sign: i32 = if curr_atom_data.f_charge < 0 { 1 } else { 0 };
    let heavy_atom_connections = curr_atom_data.element.valence_electrons() - f_charge as i32;

    let concatenated = format!("{}{}{}{}{}", atomic_number, h_count, f_charge, sign, heavy_atom_connections);
    djb2_hash(&concatenated)
}




pub fn ecfp(molecule: Molecule) -> HashSet<u64>{
    let mut atom_identifier_set: HashSet<u64> =  HashSet::new();
    let mut mega_bond_list = Vec::new();
    
    for atom in molecule.atoms.iter() {
        let mut bond_list = Vec::new();
        let hashed_atom = compute_atom_hashes(atom);
        atom_identifier_set.insert(hashed_atom);
        let neighbours = molecule.get_neighbouring_bond_indexes(atom.index);
        for nei in neighbours.iter() {
            let bond = molecule.bonds.get(*nei).unwrap();
            bond_list.push(bond);
        }

        bond_list.sort_by(|a, b| {
            match a.bond_data.bond_order.partial_cmp(&b.bond_data.bond_order) {
                Some(Ordering::Equal) => a.target.cmp(&b.target),
                other_ordering => other_ordering.unwrap_or(Ordering::Equal),
            }
        });

        mega_bond_list.push(bond_list);
    }

    for (i, bond_list) in mega_bond_list.iter().enumerate() {
        let mut iteration_index = 1;
        for bond in bond_list {
            let src_atom_hash = *atom_identifier_set.get(&compute_atom_hashes(&molecule.atoms[i])).unwrap();
            let dest_atom_hash = *atom_identifier_set.get(&compute_atom_hashes(&molecule.atoms[bond.target])).unwrap();

            let feature_str = format!(
                "{},{},{},{},{},{}",
                iteration_index,
                src_atom_hash,
                bond.bond_data.bond_order,
                dest_atom_hash,
                bond.bond_data.bond_order,
                src_atom_hash
            );

            let feature_string_hash = djb2_hash(&feature_str);

            atom_identifier_set.insert(feature_string_hash);
            iteration_index += 1;
        }
    }
    return atom_identifier_set;
}

