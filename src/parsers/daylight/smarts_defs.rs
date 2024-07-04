pub enum OpCode {
    SeedAtom,
    GrowBond,
    CloseRing,
    SamePart,
    DiffPart,
    RxnRole,
    TetraLeft,
    TetraRight,
}

pub enum NodeData {
    Atom(AtomExpr),
    Bond(BondExpr),
    None,
}

pub struct TreeNode {
    pub op_code: OpCode,
    pub data: NodeData,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

pub enum AtomExpr {
    Leaf {
        atom_type: i16,
        value: i32,
    },
    Recursive {
        atom_type: i16,
        recur: usize,
    },
    Monadic {
        atom_type: i16,
        arg: usize,
    },
    Binary {
        atom_type: i16,
        left: usize,
        right: usize,
    },
}

pub enum BondExpr {
    Monadic {
        bond_type: i16,
        arg: usize,
    },
    Binary {
        bond_type: i16,
        left: usize,
        right: usize,
    },
}

pub struct SmartsPattern {
    pub nodes: Vec<TreeNode>,
    pub root: i16,
    pub smarts_string: String,
}