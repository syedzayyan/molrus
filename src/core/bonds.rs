#[derive(PartialEq, Clone, Debug)] 
pub enum BondType {
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
            Self::Single => Self::Single,
            Self::Double => Self::Double,
            Self::Triple => Self::Triple,
            Self::Quadruple => Self::Quadruple,
            Self::Aromatic => Self::Aromatic,
            Self::Up => Self::Down,
            Self::Down => Self::Up
        }
    }

    pub fn get_bond_order(&self) -> f32{
        match self {
            Self::Single => 1.0,
            Self::Double => 2.0,
            Self::Triple => 3.0,
            Self::Quadruple => 4.0,
            Self::Aromatic => 1.5,
            Self::Up => 2.0,
            Self::Down => 2.0
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
