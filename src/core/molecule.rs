// Code Stolen from https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use super::{atoms::{Atom, AtomData, AtomIndex}, bonds::{Bond, BondData, BondIndex}};

// Graph Related Functions
#[derive(Clone)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl Molecule {
    pub fn new() -> Molecule {
        Molecule {
            atoms: Vec::new(),
            bonds: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom_data : AtomData) -> AtomIndex {
        let index = self.atoms.len();
        self.atoms.push(Atom {
            index: index,
            outgoing_bond: None,
            atom_data: atom_data,
        });
        index
    }

    pub fn add_bond(&mut self, source: AtomIndex, target: AtomIndex, bond_data: BondData) {
        let bond_index = self.bonds.len();
        let atom_data: &mut _ = &mut self.atoms[source];
        self.bonds.push(Bond {
            target: target,
            next_outgoing_bond: atom_data.outgoing_bond,
            bond_data: bond_data
        });
        atom_data.outgoing_bond = Some(bond_index);
    }

    pub fn neighbours(&self, source: AtomIndex) -> Neighbours {
        let first_outgoing_edge = self.atoms[source].outgoing_bond;
        Neighbours {
            molecule: self,
            current_edge_index: first_outgoing_edge,
        }
    }
    pub fn get_neighbouring_bond_indexes(&self, atom_index: AtomIndex) -> Vec<usize> {
        let mut bond_indexes = Vec::new();
        let atom = &self.atoms[atom_index];
    
        if let Some(first_bond_index) = atom.outgoing_bond {
            let mut current_bond_index = Some(first_bond_index);
    
            while let Some(bond_index) = current_bond_index {
                let bond = &self.bonds[bond_index];
                if bond.target != atom_index {
                    bond_indexes.push(bond_index);
                }
                current_bond_index = bond.next_outgoing_bond.clone();
            }
        }
        bond_indexes
    }

    pub fn get_num_bonds(&self, atom_index: AtomIndex) -> f32 {
        let mut bond_sum = 0.0;
        let neighbours = self.get_neighbouring_bond_indexes(atom_index);
        for nei in neighbours {
            let bond = &self.bonds[nei];
            bond_sum += bond.bond_data.bond_order as f32;
        }
        bond_sum
    }
    pub fn h_count_update(&mut self, atom_index: AtomIndex, h_count: u8) {
        self.atoms[atom_index].atom_data.h_count_update(h_count);
    }
}

pub struct Neighbours<'molecule> {
    molecule: &'molecule Molecule,
    current_edge_index: Option<BondIndex>,
}

impl<'molecule> Iterator for Neighbours<'molecule> {
    type Item = AtomIndex;

    fn next(&mut self) -> Option<AtomIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let bond = &self.molecule.bonds[edge_num];
                self.current_edge_index = bond.next_outgoing_bond;
                Some(bond.target)
            }
        }
    }
}
