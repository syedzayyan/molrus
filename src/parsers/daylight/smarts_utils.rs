use crate::{core::defs::{Atom, Bond}, parsers::{error::Error, scanner::{missing_character, Scanner}}};
use super::smarts_defs::{Expr, ExprType, SmartsPattern};

fn parse_primitive_bond_types(scanner: & mut Scanner) -> Option<Expr> {
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
        _ => return None
    };
    Some(Expr {
        expr_type : bond_expr,
        val : None,
        left : None,
        right: None
    })
}

pub fn parse_bond_expr(level: u8, scanner: & mut Scanner) -> Result<Expr, Error> {
    match level {
        // low_precedence_conjunction
        0 => {
            let mut expr = parse_bond_expr(1, scanner)?;
            while let Some(';') = scanner.peek() {
                let right = parse_bond_expr(1, scanner)?;
                expr = Expr {
                    expr_type: ExprType::BeAndlo,
                    val: None,
                    left: Some(Box::new(expr)),
                    right: Some(Box::new(right)),
                };
            }
            Ok(expr)
        },
        // Disjunction
        1 => {
            let mut expr = parse_bond_expr(2, scanner)?;
            while let Some(',') = scanner.peek() {
                scanner.pop();
                let right = parse_bond_expr(2, scanner)?;
                expr = Expr {
                    expr_type: ExprType::BeOr,
                    val: None,
                    left: Some(Box::new(expr)),
                    right: Some(Box::new(right)),
                };
            }
            Ok(expr)
        },
        2 => {
            let mut expr = parse_bond_expr(1, scanner)?;
            while !matches!(scanner.peek(), Some(']' | ';' | ',') | None) {
                if let Some('&') = scanner.peek() {
                    scanner.pop();
                }
                let prev_cursor = scanner.cursor();
                match parse_bond_expr(3, scanner) {
                    Ok(right) => {
                        expr = Expr {
                            expr_type: ExprType::BeAndhi,
                            val: None,
                            left: Some(Box::new(expr)),
                            right: Some(Box::new(right)),
                        };
                    }
                    Err(_) if prev_cursor == scanner.cursor() => break,
                    Err(e) => return Err(e),
                }
            }
            Ok(expr)
        },
        3 => {
            if let Some('!') = scanner.peek() {
                scanner.pop();
                let expr = parse_bond_expr( 3, scanner)?;
                Ok(Expr {
                    expr_type: ExprType::BeNot,
                    val: None,
                    left: Some(Box::new(expr)),
                    right: None,
                })
            } else {
                Ok(parse_primitive_bond_types(scanner).unwrap())
            }
        },
        _ => {
            Ok(Expr {
                expr_type : ExprType::BeAny,
                val: None,
                left: None,
                right:None
            })
        }
    }
}

pub fn parse_atom_expr (scanner: &mut Scanner, smartsins : &mut SmartsPattern, prev_node: &mut Option<usize>) -> Result<Expr, Error> {
    let mut atomic_num: Option<i8> = None;
    let mut expr_type = ExprType::AeAliphatic;
    while let Some(_) = scanner.peek() {
        match scanner.peek() {
            Some('#') => {
                scanner.pop();
                match scanner.peek() {
                    Some('0'..='9') => {
                        atomic_num = Some(scanner.peek().unwrap().to_owned() as i8);
                    },
                    _ => return Err (missing_character(scanner))
                }
            },
            Some('$') => {
                let recursive_smarts = collect_recursive_smarts(scanner);
                let mut recursive_smart_instance = SmartsPattern::new(recursive_smarts);
                recursive_smart_instance.build_ast()?;
                let mut recur_nodes = recursive_smart_instance.nodes;
                recur_nodes[0].data.expr_type = ExprType::AeRecur;
                smartsins.nodes.extend(recur_nodes);
                *prev_node = Some(smartsins.nodes.len());
            }
            Some('*') => expr_type = ExprType::True,
            Some('+') => {
                expr_type = ExprType::AeCharge;
                match scanner.peek() {
                    Some('0'..='9') => {
                        atomic_num = Some(scanner.peek().unwrap().to_owned() as i8);
                    },
                    _ => return Err (missing_character(scanner))
                }
            },
            Some('-') => {
                expr_type = ExprType::AeCharge;
                match scanner.peek() {
                    Some('0'..='9') => {
                        atomic_num = Some((scanner.peek().unwrap().to_owned() as i8) * -1);
                    },
                    _ => return Err (missing_character(scanner))
                }
            },
            Some('@') => {

            }
            _ => todo!()
        }
    }
    Ok(Expr {
        expr_type : expr_type,
        val: atomic_num,
        left: None,
        right:None
    })
}

pub fn collect_recursive_smarts(scanner: &mut Scanner) -> String {
    let mut nested_smarts = String::new();
    while let Some(ch) = scanner.peek() {
        match ch {
            '(' => {
                scanner.pop();
            },
            ')' => {
                scanner.pop();
                break;
            },
            _ => {
                nested_smarts.push(*ch);
                scanner.pop();
            },
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
            ExprType::AeAromelem => return current_expr.val == Some(atom.element as i8) && atom.aromatic,
            ExprType::AeAliphelem => return current_expr.val == Some(atom.element as i8) && !atom.aromatic,
            ExprType::AeHcount => {
                // Assuming we have a method to get implicit hydrogen count
                let total_h = atom.hydrogens;
                return current_expr.val == Some(total_h as i8);
            },
            ExprType::AeCharge => return current_expr.val == Some(atom.f_charge),
            ExprType::AeConnect => {
                // Assuming we have a method to get total degree
                // return current_expr.val == Some(atom.get_total_degree() as i8);
            },
            ExprType::AeDegree => return current_expr.val == Some(atom.outgoing_bond.len() as i8),
            ExprType::AeImplicit => {
                // Assuming we have a method to get implicit hydrogen count
                // return current_expr.val == Some(atom.get_implicit_h_count() as i8);
            },
            ExprType::AeRings => {
                // Assuming we have a method to get ring count
                // return current_expr.val == Some(atom.get_ring_count() as i8);
            },
            ExprType::AeSize => {
                // Assuming we have a method to check ring size
                // return atom.is_in_ring_size(current_expr.val.unwrap() as usize);
            },
            ExprType::AeValence => {
                // Assuming we have a method to get total valence
                // return current_expr.val == Some(atom.get_total_valence() as i8);
            },
            ExprType::AeChiral => return true, // Always return true and check later
            ExprType::AeHyb => {
                // Assuming we have a method to get hybridization
                // return current_expr.val == Some(atom.get_hyb() as i8);
            },
            ExprType::AeRingconnect => {
                // Assuming we have a method to count ring bonds
                // return current_expr.val == Some(atom.count_ring_bonds() as i8);
            },
            ExprType::AeNot => {
                return !eval_atom_expr(current_expr.left.as_ref().unwrap(), atom);
            },
            ExprType::AeAndhi | ExprType::AeAndlo => {
                if !eval_atom_expr(current_expr.left.as_ref().unwrap(), atom) {
                    return false;
                }
                current_expr = current_expr.right.as_ref().unwrap();
            },
            ExprType::AeOr => {
                if eval_atom_expr(current_expr.left.as_ref().unwrap(), atom) {
                    return true;
                }
                current_expr = current_expr.right.as_ref().unwrap();
            },
            ExprType::AeRecur => {
                // This part involves caching and recursive matching,
                // which would require additional context and implementation.
                // For now, we'll return false as a placeholder.
                return false;
            },
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
            },
            ExprType::BeOr => {
                if eval_bond_expr(current_expr.left.as_ref().unwrap(), bond) {
                    return true;
                }
                current_expr = current_expr.right.as_ref().unwrap();
            },
            ExprType::BeNot => {
                return !eval_bond_expr(current_expr.left.as_ref().unwrap(), bond);
            },
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