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
pub enum AtomTypes {
    AeAndhi,          // AND logical operation, high priority
    AeAndlo,          // AND logical operation, low priority
    AeOr,             // OR logical operation
    AeRecur,          // Recursive condition (used in substructure searching)
    AeNot,            // NOT logical operation
    AeTrue,           // True condition
    AeFalse,          // False condition
    AeAromatic,       // Aromatic atom
    AeAliphatic,      // Aliphatic (non-aromatic) atom
    AeCyclic,         // Cyclic (part of a ring) atom
    AeAcyclic,        // Acyclic (not part of a ring) atom
    AeMass,           // Atomic mass condition
    AeElem,           // Element type (atomic number)
    AeAromelem,       // Aromatic element
    AeAliphelem,      // Aliphatic element
    AeHcount,         // Hydrogen count condition
    AeCharge,         // Formal charge condition
    AeConnect,        // Connectivity (number of connections to other atoms)
    AeDegree,         // Degree (number of directly bonded atoms)
    AeImplicit,       // Implicit hydrogens count
    AeRings,          // Number of rings the atom is part of
    AeSize,           // Ring size
    AeValence,        // Valence (number of electrons in the outer shell)
    AeChiral,         // Chirality (handedness) condition
    AeHyb,            // Hybridization state (sp, sp2, sp3, etc.)
    AeRingconnect,    // Number of ring connections
    AlClockwise,      // Clockwise chiral configuration
    AlAnticlockwise,  // Anticlockwise chiral configuration
    AlUnspecified,    // Unspecified chirality
}

pub enum AtomExpr {
    // Terminal Atoms like the end of a branch
    Leaf {
        atom_type: AtomTypes,
        value: i32,
    },
    // For labelling Recursive SMARTS Atoms
    Recursive {
        atom_type: AtomTypes,
        recur: Option<Box<AtomExpr>>,
    },
    // For labelling atoms like seed atoms
    Monadic {
        atom_type: AtomTypes,
        arg: Option<Box<AtomExpr>>,
    },
    // For labelling normal atoms because atoms will usually you know they will lead to more branches of a tree
    Binary {
        atom_type: AtomTypes,
        left: Option<Box<AtomExpr>>,
        right: Option<Box<AtomExpr>>,
    },
}
#[derive(Copy, Clone)]
pub enum BondTypes {
    BeAndhi,        // AND logical operation, high priority
    BeAndlo,        // AND logical operation, low priority
    BeOr,           // OR logical operation
    BeNot,          // NOT logical operation
    BeAny,          // Any bond type
    BeDefault,      // Default bond type
    BeSingle,       // Single bond
    BeDouble,       // Double bond
    BeTriple,       // Triple bond
    BeQuad,         // Quadruple bond
    BeArom,         // Aromatic bond
    BeRing,         // Ring bond
    BeUp,           // Upward bond (stereochemistry)
    BeDown,         // Downward bond (stereochemistry)
    BeUpunspec,     // Upward bond, unspecified stereochemistry
    BeDownunspec,   // Downward bond, unspecified stereochemistry
}

#[derive(Clone)]
pub enum BondExpr {
    // Bonds that don't really lead to other atoms/bonds, because why would they?
    Monadic {
        bond_type: BondTypes,
        arg: Option<Box<BondExpr>>,
    },
    // Bond that leads to branches. For now forget about it
    Binary {
        bond_type: BondTypes,
        left: Option<Box<BondExpr>>,
        right: Option<Box<BondExpr>>,
    },
}

pub struct AtomSpec {
    pub atom_expr : AtomExpr,
    pub visit: usize,
    pub part: usize,
    pub chiral_flag: bool,
    pub vb : usize,
    pub nbrs : Vec<usize>
}
pub struct BondSpec {
    pub bond_expr: BondExpr,
    pub src : usize,
    pub dest: usize,
    pub visit: usize,
    pub grow : bool,
}
pub enum NodeData {
    Atom(AtomSpec),
    Bond(BondSpec),
    Unknown,
}

pub struct TreeNode {
    pub op_code: OpCode,
    pub data: NodeData,
    pub parent: usize,
    pub visit: bool,
}



pub struct SmartsPattern {
    pub nodes: Vec<TreeNode>,
    pub root: usize,
    pub smarts_string: String,
    pub chirality: bool
}
