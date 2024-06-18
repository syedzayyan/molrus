#[derive(PartialEq, Clone, Debug)] 
pub struct BondData {
    pub ring: bool,
    pub bond_order: f32,
    pub axialness: i32 // 0 for down, 1 for up and 2 for equatorial bond
}

pub type BondIndex = usize;

#[derive(PartialEq, Clone)]
pub struct Bond {
    pub target: BondIndex,
    pub next_outgoing_bond: Option<BondIndex>,
    pub bond_data: BondData
}
