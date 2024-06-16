use crate::core::mendeleev::Element;

use super::{error::Error, scanner::{missing_character, Scanner}};

pub fn read_symbol(scanner: &mut Scanner) -> Result<Element, Error> {
    match scanner.peek() {
        Some('A') => {
            scanner.pop();
            match scanner.peek() {
                Some('c') => Ok(Element::Ac),
                Some('g') => Ok(Element::Ag),
                Some('l') => Ok(Element::Al),
                Some('m') => Ok(Element::Am),
                Some('r') => Ok(Element::Ar),
                Some('s') => Ok(Element::As),
                Some('t') => Ok(Element::At),
                Some('u') => Ok(Element::Au),
                _ => Err(missing_character(scanner))
            }
        },
        Some('B') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::Ba),
                Some('e') => Ok(Element::Be),
                Some('h') => Ok(Element::Bh),
                Some('i') => Ok(Element::Bi),
                Some('k') => Ok(Element::Bk),
                Some('r') => Ok(Element::Br),
                _ => Ok(Element::B)
            }
        },
        Some('C') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::Ca),
                Some('d') => Ok(Element::Cd),
                Some('e') => Ok(Element::Ce),
                Some('f') => Ok(Element::Cf),
                Some('l') => Ok(Element::Cl),
                Some('m') => Ok(Element::Cm),
                Some('n') => Ok(Element::Cn),
                Some('o') => Ok(Element::Co),
                Some('r') => Ok(Element::Cr),
                Some('s') => Ok(Element::Cs),
                Some('u') => Ok(Element::Cu),
                _ => Ok(Element::C)
            }
        },
        Some('D') => {
            scanner.pop();
            match scanner.peek() {
                Some('b') => Ok(Element::Db),
                Some('s') => Ok(Element::Ds),
                Some('y') => Ok(Element::Dy),
                _ => Err(missing_character(scanner))
            }
        },
        Some('E') => {
            scanner.pop();

            match scanner.peek() {
                Some('r') => Ok(Element::Er),
                Some('s') => Ok(Element::Es),
                Some('u') => Ok(Element::Eu),
                _ => Err(missing_character(scanner))
            }
        },
        Some('F') => {
            scanner.pop();

            match scanner.peek() {
                Some('e') => Ok(Element::Fe),
                Some('l') => Ok(Element::Fl),
                Some('m') => Ok(Element::Fm),
                Some('r') => Ok(Element::Fr),
                _ => Ok(Element::F)
            }
        },
        Some('G') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::Ga),
                Some('d') => Ok(Element::Gd),
                Some('e') => Ok(Element::Ge),
                _ => Ok(Element::F)
            }
        },
        Some('H') => {
            scanner.pop();

            match scanner.peek() {
                Some('e') => Ok(Element::He),
                Some('f') => Ok(Element::Hf),
                Some('g') => Ok(Element::Hg),
                Some('o') => Ok(Element::Ho),
                Some('s') => Ok(Element::Hs),
                _ => Ok(Element::H)
            }
        },
        Some('I') => {
            scanner.pop();

            match scanner.peek() {
                Some('n') => Ok(Element::In),
                Some('r') => Ok(Element::Ir),
                _ => Ok(Element::I)
            }
        },
        Some('K') => {
            scanner.pop();

            match scanner.peek() {
                Some('r') => Ok(Element::Kr),
                _ => Ok(Element::K)
            }
        },
        Some('L') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::La),
                Some('i') => Ok(Element::Li),
                Some('r') => Ok(Element::Lr),
                Some('u') => Ok(Element::Lu),
                Some('v') => Ok(Element::Lv),
                _ => Err(missing_character(scanner))
            }
        },
        Some('M') => {
            scanner.pop();

            match scanner.peek() {
                Some('c') => Ok(Element::Mc),
                Some('d') => Ok(Element::Md),
                Some('g') => Ok(Element::Mg),
                Some('n') => Ok(Element::Mn),
                Some('o') => Ok(Element::Mo),
                Some('t') => Ok(Element::Mt),
                _ => Err(missing_character(scanner))
            }
        },
        Some('N') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::Na),
                Some('b') => Ok(Element::Nb),
                Some('d') => Ok(Element::Nd),
                Some('e') => Ok(Element::Ne),
                Some('h') => Ok(Element::Nh),
                Some('i') => Ok(Element::Ni),
                Some('o') => Ok(Element::No),
                Some('p') => Ok(Element::Np),
                _ => Ok(Element::N)
            }
        },
        Some('O') => {
            scanner.pop();

            match scanner.peek() {
                Some('g') => Ok(Element::Og),
                Some('s') => Ok(Element::Os),
                _ => Ok(Element::O)
            }
        },
        Some('P') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::Pa),
                Some('b') => Ok(Element::Pb),
                Some('d') => Ok(Element::Pd),
                Some('m') => Ok(Element::Pm),
                Some('o') => Ok(Element::Po),
                Some('r') => Ok(Element::Pr),
                Some('t') => Ok(Element::Pt),
                Some('u') => Ok(Element::Pu),
                _ => Ok(Element::P)
            }
        },
        Some('R') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::Ra),
                Some('b') => Ok(Element::Rb),
                Some('e') => Ok(Element::Re),
                Some('f') => Ok(Element::Rf),
                Some('g') => Ok(Element::Rg),
                Some('h') => Ok(Element::Rh),
                Some('n') => Ok(Element::Rn),
                Some('u') => Ok(Element::Ru),
                _ => Err(missing_character(scanner))
            }
        },
        Some('S') => {
            scanner.pop();

            match scanner.peek() {
                Some('b') => Ok(Element::Sb),
                Some('c') => Ok(Element::Sc),
                Some('e') => Ok(Element::Se),
                Some('g') => Ok(Element::Sg),
                Some('i') => Ok(Element::Si),
                Some('m') => Ok(Element::Sm),
                Some('n') => Ok(Element::Sn),
                Some('r') => Ok(Element::Sr),
                _ => Ok(Element::S)
            }
        },
        Some('T') => {
            scanner.pop();

            match scanner.peek() {
                Some('a') => Ok(Element::Ta),
                Some('b') => Ok(Element::Tb),
                Some('c') => Ok(Element::Tc),
                Some('e') => Ok(Element::Te),
                Some('h') => Ok(Element::Th),
                Some('i') => Ok(Element::Ti),
                Some('l') => Ok(Element::Tl),
                Some('m') => Ok(Element::Tm),
                Some('s') => Ok(Element::Ts),
                _ => Err(missing_character(scanner))
            }
        },
        Some('U') => Ok(Element::U),
        Some('V') => Ok(Element::V),
        Some('W') => Ok(Element::W),
        Some('X') => {
            scanner.pop();

            match scanner.peek() {
                Some('e') => Ok(Element::Xe),
                _ => Err(missing_character(scanner))
            }
        },
        Some('Y') => {
            scanner.pop();

            match scanner.peek() {
                Some('b') => Ok(Element::Yb),
                _ => Ok(Element::Y)
            }
        },
        Some('Z') => {
            scanner.pop();

            match scanner.peek() {
                Some('n') => Ok(Element::Zn),
                Some('r') => Ok(Element::Zr),
                _ => Err(missing_character(scanner))
            }
        },
        _ => Err(missing_character(scanner))
    }
}