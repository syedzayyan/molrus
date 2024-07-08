// Code Stolen from https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
use super::defs::{Atom, Bond};

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
    pub fn add_atom(&mut self, atom: Atom){
        self.atoms.push(atom)
    }
    pub fn add_bond(&mut self, bond: Bond){
        self.bonds.push(bond)
    }
    pub fn h_count_update(&mut self, atom_index: usize, h_count: usize) {
        self.atoms[atom_index].h_count_update(h_count);
    }

    pub fn get_bond(&self, atom1: usize, atom2: usize) -> Option<&Bond> {
        self.bonds.iter().find(|&bond| 
            (bond.source == atom1 && bond.dest == atom2) || 
            (bond.source == atom2 && bond.dest == atom1)
        )
    }
}