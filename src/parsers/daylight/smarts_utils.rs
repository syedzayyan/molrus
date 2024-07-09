use crate::parsers::{error::Error, scanner::{missing_character, Scanner}};
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