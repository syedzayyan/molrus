use crate::core::{defs::{Atom, Bond}, molecule::Molecule};

impl Molecule {
    pub fn mol_to_smiles(&self) {
        let atom = &self.atoms;
        // let do_you_need_brackets = self
    }
}



// Utilities
fn atom_to_smi_str(atom: Atom) -> String {
    let needs_brackets: bool = atom.isotope != 0
        || atom.hydrogens != 0
        || atom.f_charge != 0
        || atom.aromatic
        || atom.ring;

    if needs_brackets {
        let mut atom_str = String::from("[");
        if atom.isotope != 0 {
            atom_str.push_str(&atom.isotope.to_string());
        }
        atom_str.push_str(&atom.element.to_string());
        if atom.aromatic {
            atom_str.push_str(":");
        }
        if atom.hydrogens > 0 {
            atom_str.push_str(&format!("H{}", atom.hydrogens));
        }
        if atom.f_charge > 0 {
            atom_str.push_str(&format!("+{}", atom.f_charge));
        } else if atom.f_charge < 0 {
            atom_str.push_str(&format!("{}", atom.f_charge));
        }
        atom_str.push(']');
        return atom_str;
    } else {
        return atom.element.to_string();
    }
}

fn bond_to_smiles_str(bond: Bond) -> String {
    match bond.bond_order {
        1 => "-",
        2 => "=",
        3 => "#",
        4 => "$",
        _ => "",
    }.to_string()
}
