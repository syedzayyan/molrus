use super::configuration::Configuration;
use super::{bonds::BondIndex, mendeleev::Element};

// Defines properties of a atom
#[derive(Clone)]
pub struct AtomData {
    /// The element of the atom.
    pub element: Element,
    /// The isotope of the atom.
    pub isotope: Option<i8>,
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
    pub symmetry_class: u8,
}

impl AtomData {
    pub fn aromatic_reverse (&mut self) {
        self.aromatic = !self.aromatic;
    }
    pub fn ring_reverse (&mut self) {
        self.ring = !self.ring;
    }
    pub fn sym_class_update (&mut self, sym : u8) {
        self.symmetry_class = sym;
    }
    pub fn h_count_update (&mut self, h_count : u8) {
        self.hydrogens = h_count;
    }
}

pub type AtomIndex = usize;

pub struct Atom {
    pub outgoing_bond: Option<BondIndex>,
    pub atom_data: AtomData,
}