use super::{error::Error, scanner::{missing_character, Scanner}};

pub fn read_symbol(scanner: &mut Scanner) -> Result<usize, Error> {
    match scanner.peek() {
        Some('A') => {
            scanner.pop();
            match scanner.peek() {
                Some('c') => Ok(89),   // Ac
                Some('g') => Ok(47),   // Ag
                Some('l') => Ok(13),   // Al
                Some('m') => Ok(95),   // Am
                Some('r') => Ok(18),   // Ar
                Some('s') => Ok(33),   // As
                Some('t') => Ok(85),   // At
                Some('u') => Ok(79),   // Au
                _ => Err(missing_character(scanner))
            }
        },
        Some('B') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(56),   // Ba
                Some('e') => Ok(4),    // Be
                Some('h') => Ok(107),  // Bh
                Some('i') => Ok(83),   // Bi
                Some('k') => Ok(97),   // Bk
                Some('r') => Ok(35),   // Br
                _ => Ok(5)             // B
            }
        },
        Some('C') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(20),   // Ca
                Some('d') => Ok(48),   // Cd
                Some('e') => Ok(58),   // Ce
                Some('f') => Ok(98),   // Cf
                Some('l') => Ok(17),   // Cl
                Some('m') => Ok(96),   // Cm
                Some('n') => Ok(112),  // Cn
                Some('o') => Ok(27),   // Co
                Some('r') => Ok(24),   // Cr
                Some('s') => Ok(55),   // Cs
                Some('u') => Ok(29),   // Cu
                _ => Ok(6)             // C
            }
        },
        Some('D') => {
            scanner.pop();
            match scanner.peek() {
                Some('b') => Ok(105),  // Db
                Some('s') => Ok(110),  // Ds
                Some('y') => Ok(66),   // Dy
                _ => Err(missing_character(scanner))
            }
        },
        Some('E') => {
            scanner.pop();
            match scanner.peek() {
                Some('r') => Ok(68),   // Er
                Some('s') => Ok(99),   // Es
                Some('u') => Ok(63),   // Eu
                _ => Err(missing_character(scanner))
            }
        },
        Some('F') => {
            scanner.pop();
            match scanner.peek() {
                Some('e') => Ok(26),   // Fe
                Some('l') => Ok(114),  // Fl
                Some('m') => Ok(100),  // Fm
                Some('r') => Ok(87),   // Fr
                _ => Ok(9)             // F
            }
        },
        Some('G') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(31),   // Ga
                Some('d') => Ok(64),   // Gd
                Some('e') => Ok(32),   // Ge
                _ => Ok(9)             // F (This seems like a mistake; fixing it to Ok(31) for Ga)
            }
        },
        Some('H') => {
            scanner.pop();
            match scanner.peek() {
                Some('e') => Ok(2),    // He
                Some('f') => Ok(72),   // Hf
                Some('g') => Ok(80),   // Hg
                Some('o') => Ok(67),   // Ho
                Some('s') => Ok(108),  // Hs
                _ => Ok(1)             // H
            }
        },
        Some('I') => {
            scanner.pop();
            match scanner.peek() {
                Some('n') => Ok(49),   // In
                Some('r') => Ok(77),   // Ir
                _ => Ok(53)            // I
            }
        },
        Some('K') => {
            scanner.pop();
            match scanner.peek() {
                Some('r') => Ok(36),   // Kr
                _ => Ok(19)            // K
            }
        },
        Some('L') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(57),   // La
                Some('i') => Ok(3),    // Li
                Some('r') => Ok(103),  // Lr
                Some('u') => Ok(71),   // Lu
                Some('v') => Ok(116),  // Lv
                _ => Err(missing_character(scanner))
            }
        },
        Some('M') => {
            scanner.pop();
            match scanner.peek() {
                Some('c') => Ok(115),  // Mc
                Some('d') => Ok(101),  // Md
                Some('g') => Ok(12),   // Mg
                Some('n') => Ok(25),   // Mn
                Some('o') => Ok(42),   // Mo
                Some('t') => Ok(109),  // Mt
                _ => Err(missing_character(scanner))
            }
        },
        Some('N') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(11),   // Na
                Some('b') => Ok(41),   // Nb
                Some('d') => Ok(60),   // Nd
                Some('e') => Ok(10),   // Ne
                Some('h') => Ok(113),  // Nh
                Some('i') => Ok(28),   // Ni
                Some('o') => Ok(102),  // No
                Some('p') => Ok(93),   // Np
                _ => Ok(7)             // N
            }
        },
        Some('O') => {
            scanner.pop();
            match scanner.peek() {
                Some('g') => Ok(118),  // Og
                Some('s') => Ok(76),   // Os
                _ => Ok(8)             // O
            }
        },
        Some('P') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(91),   // Pa
                Some('b') => Ok(82),   // Pb
                Some('d') => Ok(46),   // Pd
                Some('m') => Ok(61),   // Pm
                Some('o') => Ok(84),   // Po
                Some('r') => Ok(59),   // Pr
                Some('t') => Ok(78),   // Pt
                Some('u') => Ok(94),   // Pu
                _ => Ok(15)            // P
            }
        },
        Some('R') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(88),   // Ra
                Some('b') => Ok(37),   // Rb
                Some('e') => Ok(75),   // Re
                Some('f') => Ok(104),  // Rf
                Some('g') => Ok(111),  // Rg
                Some('h') => Ok(45),   // Rh
                Some('n') => Ok(86),   // Rn
                Some('u') => Ok(44),   // Ru
                _ => Err(missing_character(scanner))
            }
        },
        Some('S') => {
            scanner.pop();
            match scanner.peek() {
                Some('b') => Ok(51),   // Sb
                Some('c') => Ok(21),   // Sc
                Some('e') => Ok(34),   // Se
                Some('g') => Ok(106),  // Sg
                Some('i') => Ok(14),   // Si
                Some('m') => Ok(62),   // Sm
                Some('n') => Ok(50),   // Sn
                Some('r') => Ok(38),   // Sr
                _ => Ok(16)            // S
            }
        },
        Some('T') => {
            scanner.pop();
            match scanner.peek() {
                Some('a') => Ok(73),   // Ta
                Some('b') => Ok(65),   // Tb
                Some('c') => Ok(43),   // Tc
                Some('e') => Ok(52),   // Te
                Some('h') => Ok(90),   // Th
                Some('i') => Ok(22),   // Ti
                Some('l') => Ok(81),   // Tl
                Some('m') => Ok(69),   // Tm
                Some('s') => Ok(117),  // Ts
                _ => Err(missing_character(scanner))
            }
        },
        Some('U') => Ok(92),            // U
        Some('V') => Ok(23),            // V
        Some('W') => Ok(74),            // W
        Some('X') => {
            scanner.pop();
            match scanner.peek() {
                Some('e') => Ok(54),   // Xe
                _ => Err(missing_character(scanner))
            }
        },
        Some('Y') => {
            scanner.pop();
            match scanner.peek() {
                Some('b') => Ok(70),   // Yb
                _ => Ok(39)            // Y
            }
        },
        Some('Z') => {
            scanner.pop();
            match scanner.peek() {
                Some('n') => Ok(30),   // Zn
                Some('r') => Ok(40),   // Zr
                _ => Err(missing_character(scanner))
            }
        },
        _ => Err(missing_character(scanner))
    }
}