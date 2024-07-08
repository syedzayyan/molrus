use crate::parsers::scanner::Scanner;

use super::smarts_defs::{BondExpr, BondTypes};

pub fn parse_explicit_smarts_bonds(scanner: &mut Scanner, level : u8) -> Option<BondExpr> {
    match level {
        // Low Precedence Conjunction
        0 => {
            let expr1 = parse_explicit_smarts_bonds(scanner, 1);
            match scanner.peek() {
                Some(';') => {
                    scanner.pop();
                    let expr2 = parse_explicit_smarts_bonds(scanner, 1);
                    Some(create_bond_bin_expr(BondTypes::BeAndhi, expr1, expr2))
                }
                _ => None,
            }
        },
        1 => {
            let expr1 = parse_explicit_smarts_bonds(scanner, 2);
            match scanner.peek() {
                Some(',') => {
                    scanner.pop();
                    let expr2 = parse_explicit_smarts_bonds(scanner, 2);
                    Some(create_bond_bin_expr(BondTypes::BeAndhi, expr1, expr2))
                }
                _ => None,
            } 
        },
        2 => {
            let expr1 = parse_explicit_smarts_bonds(scanner, 3);
            if let Some(exprr) = expr1 {
                None
            } else {
                match scanner.peek() {
                    Some(']') | Some(';') | Some(',') => None,
                    Some('&') => {
                        scanner.pop();
                        let expr2 = parse_explicit_smarts_bonds(scanner, 3);
                        Some(create_bond_bin_expr(BondTypes::BeAndhi, expr1, expr2))
                    }
                    _ => None
                }
            }
        },
        /* Negation or Primitive */
        3 => {
            match scanner.peek() {
                Some ('!') => {
                    scanner.pop();
                    let expr1 = parse_explicit_smarts_bonds(scanner, 3);
                    if let Some(exprr) = expr1 {
                        None
                    } else {
                        return Some(BondExpr::Monadic { bond_type: BondTypes::BeNot, arg: Some(Box::new(expr1?)) })
                    }
                },
                _ => parse_bond_primitive(scanner)
            }
        }
        _ => None,
    }
}

fn parse_bond_primitive(scanner: &mut Scanner) -> Option<BondExpr> {
    // Implement the actual parsing of primitive bond expressions.
    // For now, we'll just consume one character as a placeholder.
    if let Some(ch) = scanner.pop() {
        let bond_type = match ch {
            '-' => BondTypes::BeSingle,
            '=' => BondTypes::BeDouble,
            '#' => BondTypes::BeTriple,
            ':' => BondTypes::BeArom,
            '~' => BondTypes::BeAny,
            _ => return None,
        };
        Some(BondExpr::Monadic {
            bond_type,
            arg: None, // Placeholder
        })
    } else {
        None
    }
}

fn create_bond_bin_expr (bond_type : BondTypes, expr1 : Option<BondExpr>, expr2 : Option<BondExpr>) -> BondExpr {
    BondExpr::Binary { 
        bond_type: bond_type, left: Some(Box::new(expr1.unwrap())), right: Some(Box::new(expr2.unwrap()))
    }
}