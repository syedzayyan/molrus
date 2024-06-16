#[derive(PartialEq, Clone, Debug)] 
pub enum BondType {
    Elided,
    Single,
    Double,
    Triple,
    Quadruple,
    Aromatic,
    Up,
    Down
}
#[derive(PartialEq, Clone)] 
pub struct BondData {
    pub bond_type: BondType,
    pub ring: bool,
}

impl BondData {
    /// Returns a new BondData instance with the reversed bond type.
    pub fn reverse(&self) -> Self {
        Self {
            bond_type: self.bond_type.reverse(),
            ring: self.ring,
        }
    }
}

impl BondType {
    /// Directional bonds (Up and Down) return the complementary item.
    /// Everything else returns self.
    pub fn reverse(&self) -> Self {
        match self {
            Self::Elided => Self::Elided,
            Self::Single => Self::Single,
            Self::Double => Self::Double,
            Self::Triple => Self::Triple,
            Self::Quadruple => Self::Quadruple,
            Self::Aromatic => Self::Aromatic,
            Self::Up => Self::Down,
            Self::Down => Self::Up
        }
    }
}


pub type BondIndex = usize;

#[derive(PartialEq)]
pub struct Bond {
    pub target: BondIndex,
    pub next_outgoing_bond: Option<BondIndex>,
    pub bond_data: BondData
}
