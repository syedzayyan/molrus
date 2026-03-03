use crate::core::molecule::Molecule;
use std::collections::{HashMap, HashSet};

impl Molecule {
    pub fn mol_to_smiles(&self, canonical: bool) -> String {
        if self.atoms.is_empty() {
            return String::new();
        }

        let n = self.atoms.len();
        let mut idx = 0..n;

        // If canonical, replace default order 0..n with canonical order
        let mut canonical_indices = (0..n).collect();
        if canonical {
            canonical_indices = self.compute_canonical_order();
        }

        let mut visited = HashSet::new();
        let mut result = String::new();
        let mut ring_closures = HashMap::new(); // canonical_atom_idx -> Vec<closure_label>
        let mut closure_labels: Vec<u8> = (1..=9).collect();
        let mut next_label_idx = 0;

        let start = canonical_indices[0];
        self.visit_atom_with_indices(
            start,
            &canonical_indices,
            &mut idx,
            &mut visited,
            &mut result,
            &mut ring_closures,
            &mut closure_labels,
            &mut next_label_idx,
        );

        result
    }

    /// Simple canonical order: sort by (element, degree, aromaticity, charge)
    /// Replace this with InChI‑based labels for full standard canonical SMILES.
    fn compute_canonical_order(&self) -> Vec<usize> {
        let mut ids: Vec<usize> = (0..self.atoms.len()).collect();
        ids.sort_by(|&i, &j| {
            let a = &self.atoms[i];
            let b = &self.atoms[j];

            a.element
                .cmp(&b.element)
                .then(a.hydrogens.cmp(&b.hydrogens))
                .then(a.f_charge.cmp(&b.f_charge))
                .then(a.aromatic.cmp(&b.aromatic))
                .then_with(|| self.atom_degree(i).cmp(&self.atom_degree(j)))
                .then(i.cmp(&j))
        });
        ids
    }

    /// Degree = number of bonds, not counting order.
    fn atom_degree(&self, atom_idx: usize) -> usize {
        self.bonds
            .iter()
            .filter(|b| b.source == atom_idx || b.dest == atom_idx)
            .count()
    }

    fn visit_atom_with_indices(
        &self,
        atom_idx: usize,
        canonical_indices: &[usize],
        idx: &mut std::ops::Range<usize>,
        visited: &mut HashSet<usize>,
        output: &mut String,
        ring_closures: &mut HashMap<usize, Vec<u8>>,
        closure_labels: &mut Vec<u8>,
        next_label_idx: &mut usize,
    ) {
        let atom_idx = canonical_indices[idx.clone().next().unwrap_or(atom_idx)];
        idx.next();

        if visited.contains(&atom_idx) {
            let label =
                self.allocate_ring_label(atom_idx, ring_closures, closure_labels, next_label_idx);
            output.push_str(&label.to_string());
            return;
        }

        visited.insert(atom_idx);
        let atom_str = self.atom_to_smiles_str(atom_idx);
        output.push_str(&atom_str);

        let mut neighbors: Vec<usize> = self
            .bonds
            .iter()
            .filter(|b| b.source == atom_idx || b.dest == atom_idx)
            .map(|b| {
                if b.source == atom_idx {
                    b.dest
                } else {
                    b.source
                }
            })
            .collect();

        neighbors.sort_by(|&i, &j| {
            canonical_indices
                .iter()
                .position(|&x| x == i)
                .cmp(&canonical_indices.iter().position(|&x| x == j))
                .then(i.cmp(&j))
        });

        if neighbors.is_empty() {
            return;
        }

        for i in 0..neighbors.len() {
            let neighbor = neighbors[i];
            let bond = self.get_bond(atom_idx, neighbor).unwrap();
            let mut open_branch = false;

            if i > 0 {
                open_branch = true;
                output.push('(');
            }

            let bond_str = self.bond_order_to_smiles(bond.bond_order);
            if !bond_str.is_empty() {
                output.push_str(&bond_str);
            }

            self.visit_atom_with_indices(
                neighbor,
                canonical_indices,
                idx,
                visited,
                output,
                ring_closures,
                closure_labels,
                next_label_idx,
            );

            if open_branch {
                output.push(')');
            }
        }
    }

    fn allocate_ring_label(
        &self,
        atom_idx: usize,
        ring_closures: &mut HashMap<usize, Vec<u8>>,
        closure_labels: &mut Vec<u8>,
        next_label_idx: &mut usize,
    ) -> u8 {
        if let Some(&label) = closure_labels.get(*next_label_idx) {
            *next_label_idx += 1;
            ring_closures
                .entry(atom_idx)
                .or_insert_with(Vec::new)
                .push(label);
            label
        } else {
            *closure_labels.first().unwrap_or(&1)
        }
    }

    fn bond_order_to_smiles(&self, order: i8) -> String {
        match order {
            1 => String::new(), // often elided between organic atoms; kekulize can add where needed
            2 => "=".to_string(),
            3 => "#".to_string(),
            4 => "$".to_string(),
            _ => "".to_string(),
        }
    }
}
