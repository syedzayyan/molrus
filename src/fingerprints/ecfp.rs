use super::hash::djb2_hash;
use crate::core::{defs::Atom, mendeleev::valence_electrons, molecule::Molecule};
use std::collections::HashSet;

fn atom_feature_hash(atom: &Atom) -> u64 {
    let atomic_number = atom.element as u64;
    let h_count = atom.hydrogens as u64;
    let f_charge = atom.f_charge as i64;
    let charge_mag = f_charge.abs() as u64;
    let charge_sign = if f_charge < 0 { 1_u64 } else { 0 };

    let heavy_atom_connections = {
        let valence = valence_electrons(atom.element) as i64;
        (valence - f_charge).max(0) as u64
    };

    let s = format!(
        "{}{}{}{}{}{}",
        atomic_number, h_count, charge_mag, charge_sign, heavy_atom_connections, atom.aromatic
    );
    djb2_hash(&s)
}

pub fn ecfp(molecule: &Molecule, radius: usize) -> HashSet<u64> {
    let mut identifiers = HashSet::new();
    let n_atoms = molecule.atoms.len();

    // Step 1: initial atom identifiers (radius 0)
    let mut current_atom_ids: Vec<u64> = (0..n_atoms)
        .map(|i| atom_feature_hash(&molecule.atoms[i]))
        .collect();

    // Record all identifiers along the way (including radius 0)
    for &id in &current_atom_ids {
        identifiers.insert(id);
    }

    // Step 2: iterate for `radius` levels (Morgan expansion)
    for _ in 0..radius {
        let mut new_ids = Vec::with_capacity(n_atoms);

        for i in 0..n_atoms {
            let center_hash = current_atom_ids[i];

            let mut neighbors: Vec<u64> = molecule
                .bonds
                .iter()
                .filter(|b| b.source == i || b.dest == i)
                .map(|b| {
                    let j = if b.source == i { b.dest } else { b.source };
                    let bond_atom_hash = current_atom_ids[j];
                    let bond_order = b.bond_order as u64;
                    // Hash bond order + neighbor atom id + central atom id
                    let s = format!("{}{}{}{}", center_hash, bond_order, bond_atom_hash, b.arom);
                    djb2_hash(&s)
                })
                .collect();

            neighbors.sort_unstable(); // deterministic order

            // Combine central atom hash with sorted neighbor hashes
            let mut combined = center_hash.to_string();
            for nh in neighbors {
                combined.push_str(&nh.to_string());
            }
            let new_id = djb2_hash(&combined);

            new_ids.push(new_id);
        }

        // Step 3: accumulate all new identifiers
        for &id in &new_ids {
            identifiers.insert(id);
        }

        // Step 4: update for next radius
        current_atom_ids = new_ids;
    }

    identifiers
}

pub fn ecfp_bitvec(molecule: &Molecule, radius: usize, n_bits: usize) -> Vec<u8> {
    let ids = ecfp(molecule, radius);
    let mut vec = vec![0u8; (n_bits + 7) / 8];

    for id in ids {
        // Simple hash → bit index
        let bit_idx = (id % (n_bits as u64)) as usize;
        let byte_idx = bit_idx / 8;
        let bit = bit_idx % 8;
        vec[byte_idx] |= 1 << bit;
    }

    vec
}
