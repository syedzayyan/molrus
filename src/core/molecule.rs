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
    pub fn add_atom(&mut self, atom: Atom) {
        self.atoms.push(atom)
    }
    pub fn add_bond(&mut self, bond: Bond) {
        self.bonds.push(bond)
    }
    pub fn h_count_update(&mut self, atom_index: usize, h_count: usize) {
        self.atoms[atom_index].h_count_update(h_count);
    }

    pub fn get_bond(&self, atom1: usize, atom2: usize) -> Option<&Bond> {
        self.bonds.iter().find(|&bond| {
            (bond.source == atom1 && bond.dest == atom2)
                || (bond.source == atom2 && bond.dest == atom1)
        })
    }

    pub fn atom_to_smiles_str(&self, atom_idx: usize) -> String {
        let atom = &self.atoms[atom_idx];

        let organic = is_organic(atom.element);
        let aromatic = atom.aromatic;

        // When do we need brackets?
        let needs_brackets =
            atom.isotope != 0 || atom.hydrogens != 0 || atom.f_charge != 0 || atom.ring || !organic;

        if needs_brackets {
            let mut s = String::from("[");
            if atom.isotope != 0 {
                s.push_str(&atom.isotope.to_string());
            }

            if aromatic && organic {
                // SMILES aromatic atoms: lowercase
                let sym = match atom.element {
                    5 => "b",  // boron
                    6 => "c",  // carbon
                    7 => "n",  // nitrogen
                    8 => "o",  // oxygen
                    15 => "p", // phosphorus
                    16 => "s", // sulfur
                    _ => &atom.element.to_string(),
                };
                s.push_str(sym);
            } else {
                s.push_str(&atom.element.to_string()); // uppercase
            }

            if atom.hydrogens > 0 {
                s.push_str(&format!("H{}", atom.hydrogens));
            }
            if atom.f_charge > 0 {
                s.push_str(&format!("+{}", atom.f_charge));
            } else if atom.f_charge < 0 {
                s.push_str(&format!("{}", atom.f_charge)); // already has minus sign
            }

            s.push(']');
            return s;
        } else {
            // bare organic atom
            if organic && aromatic {
                match atom.element {
                    5 => return "b".to_string(),
                    6 => return "c".to_string(),
                    7 => return "n".to_string(),
                    8 => return "o".to_string(),
                    15 => return "p".to_string(),
                    16 => return "s".to_string(),
                    _ => {}
                }
            }
            atom.element.to_string() // uppercase
        }
    }
}

// organic subset atomic numbers: 5,6,7,8,15,16,9,17,35,53
const fn is_organic(z: usize) -> bool {
    matches!(z, 5 | 6 | 7 | 8 | 15 | 16 | 9 | 17 | 35 | 53)
}
