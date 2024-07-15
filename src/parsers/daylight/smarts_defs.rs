#[derive(Debug)]
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
// John Genius Mayfield, pointed out that I could just combine Expr Types
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ExprType {
    True,  // Any Atom
    False, // No Atom?

    AeAndhi,         // AND logical operation, high priority
    AeAndlo,         // AND logical operation, low priority
    AeOr,            // OR logical operation
    AeRecur,         // Recursive condition 
    AeNot,           // NOT logical operation
    AeAromatic,      // Aromatic atom
    AeAliphatic,     // Aliphatic (non-aromatic) atom
    AeCyclic,        // Cyclic (part of a ring) atom
    AeAcyclic,       // Acyclic (not part of a ring) atom
    AeMass,          // Atomic mass condition
    AeElem,          // Element type (atomic number)
    AeAromelem,      // Aromatic element
    AeAliphelem,     // Aliphatic element
    AeHcount,        // Hydrogen count condition
    AeCharge,        // Formal charge condition
    AeConnect,       // Connectivity (number of connections to other atoms)
    AeDegree,        // Degree (number of directly bonded atoms)
    AeImplicit,      // Implicit hydrogens count
    AeRings,         // Number of rings the atom is part of
    AeSize,          // Ring size
    AeValence,       // Valence (number of electrons in the outer shell)
    AeChiral,        // Chirality (handedness) condition
    AeHyb,           // Hybridization state (sp, sp2, sp3, etc.)
    AeRingconnect,   // Number of ring connections
    AlClockwise,     // Clockwise chiral configuration
    AlAnticlockwise, // Anticlockwise chiral configuration
    AlUnspecified,   // Unspecified chirality
    
    BeAndhi,         // AND logical operation, high priority
    BeAndlo,         // AND logical operation, low priority
    BeOr,            // OR logical operation
    BeNot,           // NOT logical operation
    BeAny,           // Any bond type
    BeDefault,       // Default bond type
    BeSingle,        // Single bond
    BeDouble,        // Double bond
    BeTriple,        // Triple bond
    BeQuad,          // Quadruple bond
    BeArom,          // Aromatic bond
    BeRing,          // Ring bond
    BeUp,            // Upward bond (stereochemistry)
    BeDown,          // Downward bond (stereochemistry)
    BeUpunspec,      // Upward bond, unspecified stereochemistry
    BeDownunspec,    // Downward bond, unspecified stereochemistry
}

#[derive(PartialEq, Clone, Debug)]
pub struct Expr {
    pub expr_type: ExprType,
    pub val: Option<i8>,
    pub left: Option<Box<Expr>>,
    pub right: Option<Box<Expr>>,
}
#[derive(Debug)]
pub struct TreeNode {
    pub op_code: OpCode,
    pub data: Expr,
    pub src: usize, 
    pub dst: Option<usize>, // Option for bonds as atoms don't really have a dst
    pub nbrs: Option<Vec<usize>>, // None for Bond Nodes
    pub visit: bool,
}
#[derive(Debug)]
pub struct SmartsPattern {
    pub nodes: Vec<TreeNode>,
    pub root: usize,
    pub smarts_string: String,
    pub chirality: bool,
    pub recursion : bool
}