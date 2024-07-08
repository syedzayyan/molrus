use crate::{core::defs::{Atom, Axialness}, parsers::{elements::read_symbol, error::Error, scanner::Scanner}};

use super::config::read_configuration;

pub fn read_bracket(scanner: &mut Scanner) -> Result<Option<Atom>, Error> {
    if let Some('[') = scanner.peek() {
        scanner.pop();
    } else {
        return Ok(None);
    }

    let isotope = read_isotope(scanner)?;
    let symbol;
    let aromatic;

    if let Some(element) = read_organic(scanner)? {
        symbol = element;
        aromatic = true;
    } else {
        symbol = read_symbol(scanner)?;
        aromatic = false;
    }

    let configuration = read_configuration(scanner)?;
    let hcount = read_hcount(scanner)?;
    let charge = read_charge(scanner)?;
    // let map: Option<i8> = read_map(scanner)?;

    match scanner.peek() {
        Some(']') => {
            scanner.pop();

            Ok(Some(Atom {
                element: symbol,
                outgoing_bond: Vec::new(),
                hydrogens: hcount,
                aromatic: aromatic,
                symmetry_class: 0,
                f_charge: charge,
                isotope: isotope,
                configuration: configuration,
                ring: false,
                coords_3d: None,
            }))
        }
        None => Err(Error::EndOfLine),
        _ => Err(Error::Character(scanner.cursor())),
    }
}

pub fn read_hcount(scanner: &mut Scanner) -> Result<usize, Error> {
    match scanner.peek() {
        Some('H') => {
            scanner.pop();
            match scanner.peek() {
                Some('0'..='9') => match scanner.pop() {
                    Some('0') => Ok(0),
                    Some('1') => Ok(1),
                    Some('2') => Ok(2),
                    Some('3') => Ok(3),
                    Some('4') => Ok(4),
                    Some('5') => Ok(5),
                    Some('6') => Ok(6),
                    Some('7') => Ok(7),
                    Some('8') => Ok(8),
                    Some('9') => Ok(9),
                    _ => Ok(1),
                },
                _ => Ok(0),
            }
        }
        _ => Ok(0),
    }
}

pub fn read_isotope(scanner: &mut Scanner) -> Result<usize, Error> {
    let mut digits = String::new();

    for _ in 0..3 {
        match scanner.peek() {
            Some('0'..='9') => digits.push(*scanner.pop().expect("digit")),
            _ => break,
        }
    }

    if digits.is_empty() {
        Ok(0)
    } else {
        Ok(digits.parse::<usize>().unwrap())
    }
}

// fn read_map(scanner: &mut Scanner) -> Result<Option<i8>, Error> {
//     match scanner.peek() {
//         Some(':') => {
//             scanner.pop();

//             let mut digits = String::new();

//             match scanner.pop() {
//                 Some(next) => if next.is_ascii_digit() {
//                     digits.push(*next);
//                 } else {
//                     return Err(Error::Character(scanner.cursor() - 1))
//                 },
//                 None => return Err(missing_character(scanner))
//             }

//             for _ in 0..2 {
//                 match scanner.peek() {
//                     Some('0'..='9') =>
//                         digits.push(*scanner.pop().expect("digit")),
//                     _ => break
//                 }
//             }

//             Ok(Some(digits.parse::<i8>().unwrap()))
//         },
//         _ => Ok(None)
//     }
// }

pub fn read_charge(scanner: &mut Scanner) -> Result<i8, Error> {
    match scanner.peek() {
        Some('+') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => Ok(value.try_into().expect("charge")),
                None => match scanner.peek() {
                    Some('+') => {
                        scanner.pop();

                        Ok(2)
                    }
                    _ => Ok(1),
                },
            }
        }
        Some('-') => {
            scanner.pop();

            match fifteen(scanner) {
                Some(value) => Ok((-value).try_into().expect("charge")),
                None => match scanner.peek() {
                    Some('-') => {
                        scanner.pop();

                        Ok(-2)
                    }
                    _ => Ok(-1),
                },
            }
        }
        _ => Ok(0),
    }
}

fn fifteen(scanner: &mut Scanner) -> Option<i8> {
    match scanner.peek() {
        Some('1'..='9') => Some(match scanner.pop() {
            Some('1') => match scanner.peek() {
                Some('1'..='5') => match scanner.pop() {
                    Some('1') => 11,
                    Some('2') => 12,
                    Some('3') => 13,
                    Some('4') => 14,
                    Some('5') => 15,
                    _ => 1,
                },
                _ => 1,
            },
            Some('2') => 2,
            Some('3') => 3,
            Some('4') => 4,
            Some('5') => 5,
            Some('6') => 6,
            Some('7') => 7,
            Some('8') => 8,
            Some('9') => 9,
            _ => unreachable!("fifteen"),
        }),
        _ => None,
    }
}

pub fn read_bond(scanner: &mut Scanner) -> i8 {
    let result = match scanner.peek() {
        Some('-') => {
            scanner.pop();
            1
        }
        Some('=') => {
            scanner.pop();
            2
        }
        Some('#') => {
            scanner.pop();
            3
        }
        Some('$') => {
            scanner.pop();
            4
        }
        Some(':') => {
            scanner.pop();
            1
        }
        _ => 1,
    };
    result
}

pub fn read_axial(scanner: &mut Scanner) -> Axialness {
    let result = match scanner.peek() {
        Some('/') => {
            scanner.pop();
            Axialness::UP
        }
        Some('\\') => {
            scanner.pop();
            Axialness::UP
        }
        _ => Axialness::UNKNOWN,
    };
    result
}

// <star> = "*"
pub fn read_star(scanner: &mut Scanner) -> Result<Option<Atom>, Error> {
    match scanner.peek() {
        Some('*') => {
            scanner.pop();

            Ok(Some(Atom {
                element: 0,
                outgoing_bond: Vec::new(),
                isotope: 0,
                hydrogens: 0,
                aromatic: false,
                f_charge: 0,
                configuration: None,
                ring: false,
                symmetry_class: 0,
                coords_3d: None,
            }))
        }
        _ => Ok(None),
    }
}
pub fn read_organic(scanner: &mut Scanner) -> Result<Option<usize>, Error> {
    match scanner.peek() {
        Some('b') => {
            scanner.pop();
            Ok(Some(5)) // Atomic number for Boron is 5
        }
        Some('c') => {
            scanner.pop();
            Ok(Some(6)) // Atomic number for Carbon is 6
        }
        Some('n') => {
            scanner.pop();
            Ok(Some(7)) // Atomic number for Nitrogen is 7
        }
        Some('o') => {
            scanner.pop();
            Ok(Some(8)) // Atomic number for Oxygen is 8
        }
        Some('p') => {
            scanner.pop();
            Ok(Some(15)) // Atomic number for Phosphorus is 15
        }
        Some('s') => {
            scanner.pop();
            Ok(Some(16)) // Atomic number for Sulfur is 16
        }
        _ => Ok(None),
    }
}
