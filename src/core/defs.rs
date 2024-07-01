use super::configuration::Configuration;
use super:: mendeleev::Element;

// Defines properties of a atom
#[derive(Clone, Debug)]
pub struct Atom {
    // List of BondIndexes
    pub outgoing_bond: Vec<BondIndex>,
    /// The element of the atom.
    pub element: Element,
    /// The isotope of the atom.
    pub isotope: i8, // 0 for none and other numbers of isotope number
    /// The number of hydrogens attached to the atom.
    pub hydrogens: u8,
    /// Indicates whether the atom is aromatic.
    pub aromatic: bool,
    /// The charge of the atom.
    pub f_charge: i8,
    /// The chirality/configuration of the atom.
    pub configuration: Option<Configuration>,
    /// part of ring?
    pub ring: bool,
    // The symmetry class of the atom
    pub symmetry_class: i8,
    // 3D Coords
    pub coords_3d: Option<(f64, f64, f64)>,
}

impl Atom {
    pub fn add_to_bond_list(&mut self, bond: BondIndex){
        self.outgoing_bond.push(bond)
    }
    pub fn aromatic_reverse(&mut self) {
        self.aromatic = !self.aromatic;
    }
    pub fn ring_reverse(&mut self) {
        self.ring = !self.ring;
    }
    pub fn sym_class_update(&mut self, sym: i8) {
        self.symmetry_class = sym;
    }
    pub fn h_count_update(&mut self, h_count: u8) {
        self.hydrogens = h_count;
    }
}

pub type BondIndex = usize;

#[derive(PartialEq, Clone)]
pub struct Bond {
    pub source: usize,
    pub dest: usize,
    pub arom: bool,
    pub ring: bool,
    pub bond_order: i8,
}