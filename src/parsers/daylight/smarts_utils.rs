use super::smarts_defs::{Expr, ExprType};
use crate::{
    core::defs::{Atom, Bond},
    parsers::{
        error::Error,
        scanner::Scanner,
    },
};

pub fn parse_primitive_bond_types(scanner: &mut Scanner) -> Expr {
    let bond_expr = match scanner.peek() {
        Some('-') => ExprType::BeSingle,
        Some('=') => ExprType::BeDouble,
        Some('#') => ExprType::BeTriple,
        Some('$') => ExprType::BeQuad,
        Some(':') => ExprType::BeArom,
        Some('~') => ExprType::BeAny,
        Some('/') => ExprType::BeUp,
        Some('\\') => ExprType::BeDown,
        Some('@') => ExprType::BeRing,
        _ => ExprType::BeSingle,
    };
    Expr {
        expr_type: bond_expr,
        val: None,
        left: None,
        right: None,
    }
}

pub fn collect_recursive_smarts(scanner: &mut Scanner) -> String {
    let mut nested_smarts = String::new();
    while let Some(ch) = scanner.peek() {
        match ch {
            '(' => {
                scanner.pop();
            }
            ')' => {
                scanner.pop();
                break;
            }
            _ => {
                nested_smarts.push(*ch);
                scanner.pop();
            }
        }
    }
    nested_smarts
}

// These are utility function for matching

pub fn eval_atom_expr(expr: &Expr, atom: &Atom) -> bool {
    let mut current_expr = expr;
    loop {
        match current_expr.expr_type {
            ExprType::True => return true,
            ExprType::False => return false,
            ExprType::AeAromatic => return atom.aromatic,
            ExprType::AeAliphatic => return !atom.aromatic,
            ExprType::AeCyclic => return atom.ring,
            ExprType::AeAcyclic => return !atom.ring,
            ExprType::AeMass => return current_expr.val == Some(atom.isotope as i8),
            ExprType::AeElem => return current_expr.val == Some(atom.element as i8),
            ExprType::AeAromelem => {
                return current_expr.val == Some(atom.element as i8) && atom.aromatic
            }
            ExprType::AeAliphelem => {
                return current_expr.val == Some(atom.element as i8) && !atom.aromatic
            }
            ExprType::AeHcount => {
                // Assuming we have a method to get implicit hydrogen count
                let total_h = atom.hydrogens;
                return current_expr.val == Some(total_h as i8);
            }
            ExprType::AeCharge => return current_expr.val == Some(atom.f_charge),
            ExprType::AeConnect => {
                // Assuming we have a method to get total degree
                // return current_expr.val == Some(atom.get_total_degree() as i8);
            }
            ExprType::AeDegree => return current_expr.val == Some(atom.outgoing_bond.len() as i8),
            ExprType::AeImplicit => {
                // Assuming we have a method to get implicit hydrogen count
                // return current_expr.val == Some(atom.get_implicit_h_count() as i8);
            }
            ExprType::AeRings => {
                // Assuming we have a method to get ring count
                // return current_expr.val == Some(atom.get_ring_count() as i8);
            }
            ExprType::AeSize => {
                // Assuming we have a method to check ring size
                // return atom.is_in_ring_size(current_expr.val.unwrap() as usize);
            }
            ExprType::AeValence => {
                // Assuming we have a method to get total valence
                // return current_expr.val == Some(atom.get_total_valence() as i8);
            }
            ExprType::AeChiral => return true, // Always return true and check later
            ExprType::AeHyb => {
                // Assuming we have a method to get hybridization
                // return current_expr.val == Some(atom.get_hyb() as i8);
            }
            ExprType::AeRingconnect => {
                // Assuming we have a method to count ring bonds
                // return current_expr.val == Some(atom.count_ring_bonds() as i8);
            }
            ExprType::AeNot => {
                return !eval_atom_expr(current_expr.left.as_ref().unwrap(), atom);
            }
            ExprType::AeAndhi | ExprType::AeAndlo => {
                if !eval_atom_expr(current_expr.left.as_ref().unwrap(), atom) {
                    return false;
                }
                current_expr = current_expr.right.as_ref().unwrap();
            }
            ExprType::AeOr => {
                if eval_atom_expr(current_expr.left.as_ref().unwrap(), atom) {
                    return true;
                }
                current_expr = current_expr.right.as_ref().unwrap();
            }
            ExprType::AeRecur => {
                // This part involves caching and recursive matching,
                // which would require additional context and implementation.
                // For now, we'll return false as a placeholder.
                return false;
            }
            _ => return false,
        }
    }
}

pub fn eval_bond_expr(expr: &Expr, bond: &Bond) -> bool {
    let mut current_expr = expr;
    loop {
        match current_expr.expr_type {
            ExprType::BeAndhi | ExprType::BeAndlo => {
                if !eval_bond_expr(current_expr.left.as_ref().unwrap(), bond) {
                    return false;
                }
                current_expr = current_expr.right.as_ref().unwrap();
            }
            ExprType::BeOr => {
                if eval_bond_expr(current_expr.left.as_ref().unwrap(), bond) {
                    return true;
                }
                current_expr = current_expr.right.as_ref().unwrap();
            }
            ExprType::BeNot => {
                return !eval_bond_expr(current_expr.left.as_ref().unwrap(), bond);
            }
            ExprType::BeAny => return true,
            ExprType::BeDefault => return bond.bond_order == 1 || bond.arom,
            ExprType::BeSingle => return bond.bond_order == 1 && !bond.arom,
            ExprType::BeDouble => return bond.bond_order == 2 && !bond.arom,
            ExprType::BeTriple => return bond.bond_order == 3,
            ExprType::BeQuad => return bond.bond_order == 4,
            ExprType::BeArom => return bond.arom,
            ExprType::BeRing => return bond.ring,
            // ExprType::BeUp => return bond.is_up(),
            // ExprType::BeDown => return bond.is_down(),
            // ExprType::BeUpunspec => return !bond.is_down(),
            // ExprType::BeDownunspec => return !bond.is_up(),
            _ => return false,
        }
    }
}
