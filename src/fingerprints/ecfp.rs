// use std::{collections::VecDeque};

// use crate::core::{atoms::Atom, molecule::Molecule};

// // https://depth-first.com/articles/2019/01/11/extended-connectivity-fingerprints/



// // number of heavy atom connections
// // number of non-hydrogen bonds
// // atomic number
// // sign of charge
// // absolute charge
// // number of attached hydrogens


// fn compute_atom_hashes(atom: &Atom) -> Vec<i32> {
//     let curr_atom = atom.atom_data.element; 
//     let mut hashes: Vec<i32> = vec![
//         curr_atom.atomic_number(),
//     ];
//     hashes
// }

// pub fn ecfp(mol: Molecule, radius: i16, bits: i32){
//     // let mut stack = VecDeque::new();
//     // let mut visited = Vec::new();
//     // let atoms_ids = mol.atoms.iter().map(|atom| compute_atom_hashes(atom)).collect();
// }