use std::collections::{HashMap, VecDeque};

use crate::{
    core::molecule::Molecule,
    parsers::{error::Error, scanner::Scanner},
};

use super::{
    smarts_defs::{Expr, ExprType, OpCode, SmartsPattern, TreeNode},
    smarts_utils::{eval_atom_expr, eval_bond_expr},
};

// ────────────────────────────────────────────────────
// Free-standing combinators (not methods)
// ────────────────────────────────────────────────────

fn combine_and(lhs: Option<Expr>, rhs: Option<Expr>) -> Option<Expr> {
    match (lhs, rhs) {
        (None, None) => None,
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (Some(l), Some(r)) => Some(Expr {
            expr_type: ExprType::AeAndlo,
            val: None,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }),
    }
}

fn combine_andbond(lhs: Option<Expr>, rhs: Option<Expr>) -> Option<Expr> {
    match (lhs, rhs) {
        (None, None) => None,
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (Some(l), Some(r)) => Some(Expr {
            expr_type: ExprType::BeAndlo,
            val: None,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }),
    }
}

// ────────────────────────────────────────────────────
// Primitive parsers (free-standing)
// ────────────────────────────────────────────────────

fn parse_h_count_expr(scanner: &mut Scanner) -> Result<Expr, Error> {
    if let Some('H') = scanner.peek() {
        scanner.pop();
        let h = match scanner.peek() {
            Some('0'..='9') => match scanner.pop() {
                Some('0') => 0,
                Some('1') => 1,
                Some('2') => 2,
                Some('3') => 3,
                Some('4') => 4,
                Some('5') => 5,
                Some('6') => 6,
                Some('7') => 7,
                Some('8') => 8,
                Some('9') => 9,
                _ => unreachable!(),
            },
            _ => 1,
        };
        Ok(Expr {
            expr_type: ExprType::AeHcount,
            val: Some(h),
            left: None,
            right: None,
        })
    } else {
        Err(Error::Character(scanner.cursor()))
    }
}

fn parse_charge_expr(scanner: &mut Scanner) -> Result<Expr, Error> {
    match scanner.peek() {
        Some('+') => {
            scanner.pop();
            let charge = if let Some('+') = scanner.peek() {
                scanner.pop();
                2
            } else {
                1
            };
            Ok(Expr {
                expr_type: ExprType::AeCharge,
                val: Some(charge),
                left: None,
                right: None,
            })
        }
        Some('-') => {
            scanner.pop();
            let charge = if let Some('-') = scanner.peek() {
                scanner.pop();
                2
            } else {
                1
            };
            Ok(Expr {
                expr_type: ExprType::AeCharge,
                val: Some(-charge),
                left: None,
                right: None,
            })
        }
        _ => Ok(Expr {
            expr_type: ExprType::AeCharge,
            val: Some(0),
            left: None,
            right: None,
        }),
    }
}

fn parse_stereo_bond_expr(scanner: &mut Scanner) -> Result<Expr, Error> {
    match scanner.pop() {
        Some('/') => Ok(Expr {
            expr_type: ExprType::BeUp,
            val: None,
            left: None,
            right: None,
        }),
        Some('\\') => Ok(Expr {
            expr_type: ExprType::BeDown,
            val: None,
            left: None,
            right: None,
        }),
        _ => Err(Error::Character(scanner.cursor() - 1)),
    }
}

fn parse_bracket_atom_expr(scanner: &mut Scanner) -> Result<Expr, Error> {
    // '[' already consumed
    let mut element: Option<i8> = None;
    let mut mass: Option<i8> = None;
    let mut hcount: Option<i8> = None;
    let mut charge: Option<i8> = None;
    let mut chirality: Option<i8> = None;
    let mut connectivity: Option<i8> = None;
    let mut aromatic = false;

    loop {
        match scanner.peek() {
            Some('0'..='9') => {
                let mut digits = String::new();
                while scanner.peek().map_or(false, |c| c.is_ascii_digit()) {
                    digits.push(*scanner.pop().unwrap());
                }
                mass = Some(digits.parse::<i8>().unwrap_or(0));
            }
            Some('*') => {
                scanner.pop();
            }
            Some('H') => {
                hcount = parse_h_count_expr(scanner)?.val;
            }
            Some('+') | Some('-') => {
                charge = parse_charge_expr(scanner)?.val;
            }
            Some('@') => {
                scanner.pop();
                chirality = Some(if let Some('@') = scanner.peek() {
                    scanner.pop();
                    2
                } else {
                    1
                });
            }
            Some('c' | 'n' | 'o' | 's' | 'p' | 'b') => {
                let z = match scanner.pop().unwrap() {
                    'c' => 6,
                    'n' => 7,
                    'o' => 8,
                    's' => 16,
                    'p' => 15,
                    'b' => 5,
                    _ => unreachable!(),
                };
                element = Some(z);
                aromatic = true;
            }
            Some(c) if c.is_ascii_uppercase() => {
                let z = match scanner.pop().unwrap() {
                    'B' => 5,
                    'C' => 6,
                    'N' => 7,
                    'O' => 8,
                    'F' => 9,
                    'P' => 15,
                    'S' => 16,
                    'I' => 53,
                    _ => 0,
                };
                element = Some(z);
                aromatic = false;
            }
            Some('X') => {
                scanner.pop();
                let degree = match scanner.peek() {
                    Some('0'..='9') => match scanner.pop() {
                        Some('0') => 0,
                        Some('1') => 1,
                        Some('2') => 2,
                        Some('3') => 3,
                        Some('4') => 4,
                        Some('5') => 5,
                        Some('6') => 6,
                        Some('7') => 7,
                        Some('8') => 8,
                        Some('9') => 9,
                        _ => unreachable!(),
                    },
                    _ => 1,
                };
                connectivity = Some(degree);
            }
            Some(']') => {
                scanner.pop();
                break;
            }
            None => return Err(Error::EndOfLine),
            _ => return Err(Error::Character(scanner.cursor())),
        }
    }

    // Build expression tree from collected properties
    let mut expr = Expr {
        expr_type: ExprType::True,
        val: None,
        left: None,
        right: None,
    };

    if let Some(e) = element {
        expr = combine_and(
            Some(expr),
            Some(Expr {
                expr_type: if aromatic {
                    ExprType::AeAromelem
                } else {
                    ExprType::AeAliphelem
                },
                val: Some(e),
                left: None,
                right: None,
            }),
        )
        .unwrap();
    }

    if let Some(m) = mass {
        expr = combine_and(
            Some(expr),
            Some(Expr {
                expr_type: ExprType::AeMass,
                val: Some(m),
                left: None,
                right: None,
            }),
        )
        .unwrap();
    }

    if let Some(ch) = chirality {
        expr = combine_and(
            Some(expr),
            Some(Expr {
                expr_type: ExprType::AeChiral,
                val: Some(ch),
                left: None,
                right: None,
            }),
        )
        .unwrap();
    }

    if let Some(h) = hcount {
        expr = combine_and(
            Some(expr),
            Some(Expr {
                expr_type: ExprType::AeHcount,
                val: Some(h),
                left: None,
                right: None,
            }),
        )
        .unwrap();
    }

    if let Some(c) = charge {
        expr = combine_and(
            Some(expr),
            Some(Expr {
                expr_type: ExprType::AeCharge,
                val: Some(c),
                left: None,
                right: None,
            }),
        )
        .unwrap();
    }

    if let Some(x) = connectivity {
        expr = combine_and(
            Some(expr),
            Some(Expr {
                expr_type: ExprType::AeConnect,
                val: Some(x),
                left: None,
                right: None,
            }),
        )
        .unwrap();
    }

    Ok(expr)
}

/// Parse a SMARTS atom expression (outside brackets).
/// Stops at any token that isn't an atom primitive.
fn parse_atom_expr(scanner: &mut Scanner) -> Result<Expr, Error> {
    // Parse a single atom primitive — no implicit AND loop at top level.
    // Implicit AND chaining only happens inside [...] via parse_bracket_atom_expr.
    let expr = match scanner.peek() {
        Some('c') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAromelem,
                val: Some(6),
                left: None,
                right: None,
            }
        }
        Some('n') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAromelem,
                val: Some(7),
                left: None,
                right: None,
            }
        }
        Some('o') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAromelem,
                val: Some(8),
                left: None,
                right: None,
            }
        }
        Some('s') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAromelem,
                val: Some(16),
                left: None,
                right: None,
            }
        }
        Some('p') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAromelem,
                val: Some(15),
                left: None,
                right: None,
            }
        }
        Some('b') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAromelem,
                val: Some(5),
                left: None,
                right: None,
            }
        }
        Some('C') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphelem,
                val: Some(6),
                left: None,
                right: None,
            }
        }
        Some('N') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphelem,
                val: Some(7),
                left: None,
                right: None,
            }
        }
        Some('O') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphelem,
                val: Some(8),
                left: None,
                right: None,
            }
        }
        Some('S') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphelem,
                val: Some(16),
                left: None,
                right: None,
            }
        }
        Some('P') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphelem,
                val: Some(15),
                left: None,
                right: None,
            }
        }
        Some('B') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphelem,
                val: Some(5),
                left: None,
                right: None,
            }
        }
        Some('F') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphelem,
                val: Some(9),
                left: None,
                right: None,
            }
        }
        Some('*') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::True,
                val: None,
                left: None,
                right: None,
            }
        }
        Some('a') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAromatic,
                val: None,
                left: None,
                right: None,
            }
        }
        Some('A') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAliphatic,
                val: None,
                left: None,
                right: None,
            }
        }
        Some('r') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeCyclic,
                val: None,
                left: None,
                right: None,
            }
        }
        Some('R') => {
            scanner.pop();
            Expr {
                expr_type: ExprType::AeAcyclic,
                val: None,
                left: None,
                right: None,
            }
        }
        Some('H') => parse_h_count_expr(scanner)?,
        Some('+') | Some('-') => parse_charge_expr(scanner)?,
        Some('@') => {
            scanner.pop();
            let val = if let Some('@') = scanner.peek() {
                scanner.pop();
                2i8
            } else {
                1i8
            };
            Expr {
                expr_type: ExprType::AeChiral,
                val: Some(val),
                left: None,
                right: None,
            }
        }
        Some('[') => {
            scanner.pop();
            parse_bracket_atom_expr(scanner)?
        }
        Some('!') => {
            scanner.pop();
            let inner = parse_atom_expr(scanner)?;
            Expr {
                expr_type: ExprType::AeNot,
                val: None,
                left: Some(Box::new(inner)),
                right: None,
            }
        }
        Some(';') => {
            scanner.pop();
            let rhs = parse_atom_expr(scanner)?;
            Expr {
                expr_type: ExprType::AeAndhi,
                val: None,
                left: Some(Box::new(Expr {
                    expr_type: ExprType::True,
                    val: None,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(rhs)),
            }
        }
        Some(',') => {
            scanner.pop();
            let rhs = parse_atom_expr(scanner)?;
            Expr {
                expr_type: ExprType::AeOr,
                val: None,
                left: Some(Box::new(Expr {
                    expr_type: ExprType::True,
                    val: None,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(rhs)),
            }
        }
        _ => Expr {
            expr_type: ExprType::True,
            val: None,
            left: None,
            right: None,
        },
    };

    Ok(expr)
}

/// Parse a SMARTS bond expression.
fn parse_bond_expr(scanner: &mut Scanner) -> Result<Expr, Error> {
    let mut result: Option<Expr> = None;

    loop {
        let expr = match scanner.peek() {
            Some('-') => {
                scanner.pop();
                Expr {
                    expr_type: ExprType::BeSingle,
                    val: None,
                    left: None,
                    right: None,
                }
            }
            Some('=') => {
                scanner.pop();
                Expr {
                    expr_type: ExprType::BeDouble,
                    val: None,
                    left: None,
                    right: None,
                }
            }
            Some('#') => {
                scanner.pop();
                Expr {
                    expr_type: ExprType::BeTriple,
                    val: None,
                    left: None,
                    right: None,
                }
            }
            Some('$') => {
                scanner.pop();
                Expr {
                    expr_type: ExprType::BeQuad,
                    val: None,
                    left: None,
                    right: None,
                }
            }
            Some(':') => {
                scanner.pop();
                Expr {
                    expr_type: ExprType::BeArom,
                    val: None,
                    left: None,
                    right: None,
                }
            }
            Some('~') => {
                scanner.pop();
                Expr {
                    expr_type: ExprType::BeAny,
                    val: None,
                    left: None,
                    right: None,
                }
            }
            Some('@') => {
                scanner.pop();
                let val = if let Some('@') = scanner.peek() {
                    scanner.pop();
                    2i8
                } else {
                    1i8
                };
                Expr {
                    expr_type: ExprType::AeChiral,
                    val: Some(val),
                    left: None,
                    right: None,
                }
            }
            Some('/') | Some('\\') => parse_stereo_bond_expr(scanner)?,
            Some('!') => {
                scanner.pop();
                let inner = parse_bond_expr(scanner)?;
                Expr {
                    expr_type: ExprType::BeNot,
                    val: None,
                    left: Some(Box::new(inner)),
                    right: None,
                }
            }
            _ => break,
        };
        result = combine_andbond(result, Some(expr));
    }

    Ok(result.unwrap_or(Expr {
        expr_type: ExprType::BeDefault,
        val: None,
        left: None,
        right: None,
    }))
}

// ────────────────────────────────────────────────────
// SmartsPattern impl
// ────────────────────────────────────────────────────

impl SmartsPattern {
    pub fn new(smarts_string: &str) -> SmartsPattern {
        let mut pat = SmartsPattern {
            nodes: Vec::new(),
            root: 0,
            smarts_string: smarts_string.to_string(),
            chirality: false,
            recursion: false,
        };
        match pat.build_ast() {
            Ok(_) => {}
            Err(e) => panic!("SMARTS parse error: {:?}", e),
        }
        pat
    }

    pub fn build_ast(&mut self) -> Result<(), Error> {
        let mut scanner = Scanner::new(&self.smarts_string.clone());
        let mut branch_points: VecDeque<usize> = VecDeque::new();
        let mut prev_atom: Option<usize> = None; // index of last SeedAtom node
        let mut ring_closures: HashMap<u8, usize> = HashMap::new();
        let mut implicit_bond: Option<ExprType> = None; // type to use when no bond token seen

        while scanner.peek().is_some() {
            match scanner.peek() {
                // ── Branch open ──────────────────────────────────────────────
                Some('(') => {
                    branch_points.push_back(prev_atom.expect("branch without atom"));
                    scanner.pop();
                }

                // ── Branch close ─────────────────────────────────────────────
                Some(')') => {
                    prev_atom = branch_points.pop_back();
                    scanner.pop();
                }

                // ── Explicit bond / stereo bond ───────────────────────────────
                Some('-') | Some('=') | Some('#') | Some('$') | Some(':') | Some('~')
                | Some('@') | Some('/') | Some('\\') | Some('!') => {
                    // Keep the bond expr to attach to the *next* atom
                    let bexpr = parse_bond_expr(&mut scanner)?;
                    implicit_bond = Some(bexpr.expr_type); // carry forward
                                                           // We push a GrowBond node later, once we know the destination atom
                                                           // Store temporarily; handled inside the atom branch below
                                                           // (see `pending_bond` approach)
                }

                // ── Ring closure digit ───────────────────────────────────────
                Some('0'..='9') => {
                    let digit = scanner.pop().unwrap().to_digit(10).unwrap() as u8;
                    self.handle_ring_closure(digit, prev_atom, &mut ring_closures)?;
                }

                // ── Two-digit ring closure (%NN) ─────────────────────────────
                Some('%') => {
                    scanner.pop();
                    let mut s = String::new();
                    for _ in 0..2 {
                        match scanner.peek() {
                            Some('0'..='9') => s.push(*scanner.pop().unwrap()),
                            _ => return Err(Error::Character(scanner.cursor())),
                        }
                    }
                    let n = s.parse::<u8>().unwrap();
                    self.handle_ring_closure(n, prev_atom, &mut ring_closures)?;
                }

                // ── Atom (any non-bond, non-branch, non-ring token) ──────────
                _ => {
                    let atom_expr = parse_atom_expr(&mut scanner)?;
                    let atom_idx = self.nodes.len();

                    self.nodes.push(TreeNode {
                        op_code: if prev_atom.is_some() {
                            OpCode::SamePart
                        } else {
                            OpCode::SeedAtom
                        },
                        data: atom_expr,
                        src: atom_idx,
                        dst: None,
                        nbrs: Some(Vec::new()),
                        visit: false,
                    });

                    // Wire up the bond from the previous atom
                    if let Some(last_atom) = prev_atom {
                        let bond_type = implicit_bond.take().unwrap_or(ExprType::BeDefault);
                        let bond_idx = self.nodes.len();

                        self.nodes.push(TreeNode {
                            op_code: OpCode::GrowBond,
                            data: Expr {
                                expr_type: bond_type,
                                val: None,
                                left: None,
                                right: None,
                            },
                            src: last_atom,
                            dst: Some(atom_idx),
                            nbrs: None,
                            visit: false,
                        });

                        // Register bond in both atom's neighbour lists
                        self.nodes[last_atom].nbrs.as_mut().unwrap().push(bond_idx);
                        self.nodes[atom_idx].nbrs.as_mut().unwrap().push(bond_idx);
                    } else {
                        self.root = atom_idx;
                    }

                    prev_atom = Some(atom_idx);
                }
            }
        }
        Ok(())
    }

    fn handle_ring_closure(
        &mut self,
        digit: u8,
        prev_atom: Option<usize>,
        ring_closures: &mut HashMap<u8, usize>,
    ) -> Result<(), Error> {
        let curr_atom = prev_atom.ok_or(Error::EndOfLine)?;

        if let Some(open_atom) = ring_closures.remove(&digit) {
            let ring_bond_idx = self.nodes.len();
            self.nodes.push(TreeNode {
                op_code: OpCode::CloseRing,
                data: Expr {
                    expr_type: ExprType::BeAny,
                    val: None,
                    left: None,
                    right: None,
                },
                src: open_atom,
                dst: Some(curr_atom),
                nbrs: None,
                visit: false,
            });
            self.nodes[open_atom]
                .nbrs
                .as_mut()
                .unwrap()
                .push(ring_bond_idx);
            self.nodes[curr_atom]
                .nbrs
                .as_mut()
                .unwrap()
                .push(ring_bond_idx);
        } else {
            ring_closures.insert(digit, curr_atom);
        }
        Ok(())
    }

    pub fn match_mol(&self, molecule: &Molecule) -> bool {
        SmartsMatch::new(self, molecule).match_smarts()
    }
}

// ────────────────────────────────────────────────────
// Substructure matcher
// ────────────────────────────────────────────────────

struct SmartsMatch<'a> {
    pattern: &'a SmartsPattern,
    molecule: &'a Molecule,
    atom_mapping: Vec<Option<usize>>,
    bond_mapping: Vec<Option<usize>>,
}

impl<'a> SmartsMatch<'a> {
    fn new(pattern: &'a SmartsPattern, molecule: &'a Molecule) -> Self {
        let n = pattern.nodes.len();
        SmartsMatch {
            pattern,
            molecule,
            atom_mapping: vec![None; n],
            bond_mapping: vec![None; n],
        }
    }

    fn match_smarts(&mut self) -> bool {
        self.match_recursive(0)
    }

    fn match_recursive(&mut self, op_index: usize) -> bool {
        if op_index == self.pattern.nodes.len() {
            return true;
        }
        let node = &self.pattern.nodes[op_index];
        match node.op_code {
            OpCode::SeedAtom | OpCode::SamePart => self.seed_atom(op_index),
            OpCode::GrowBond => {
                // If src atom isn't mapped yet, skip this bond node
                if self.atom_mapping[node.src].is_none() {
                    return self.match_recursive(op_index + 1);
                }
                self.grow_bond(op_index)
            }
            OpCode::CloseRing => self.close_ring(op_index),
            OpCode::DiffPart => self.match_recursive(op_index + 1),
            _ => false,
        }
    }

    fn seed_atom(&mut self, op_index: usize) -> bool {
        // Already placed by a grow_bond call — just advance
        if self.atom_mapping[op_index].is_some() {
            return self.match_recursive(op_index + 1);
        }

        for mol_idx in 0..self.molecule.atoms.len() {
            if self.atom_mapping.iter().any(|&m| m == Some(mol_idx)) {
                continue;
            }
            if eval_atom_expr(
                &self.pattern.nodes[op_index].data,
                &self.molecule.atoms[mol_idx],
            ) {
                self.atom_mapping[op_index] = Some(mol_idx);
                if self.match_recursive(op_index + 1) {
                    return true;
                }
                self.atom_mapping[op_index] = None;
            }
        }
        false
    }

    fn grow_bond(&mut self, op_index: usize) -> bool {
        let node = &self.pattern.nodes[op_index];
        let qry_src = node.src;
        let qry_dst = match node.dst {
            Some(d) => d,
            None => return self.match_recursive(op_index + 1),
        };

        let mol_src = match self.atom_mapping[qry_src] {
            Some(idx) => idx,
            None => return false,
        };

        // ── NEW: if dst atom already placed, just verify the bond exists ──
        if let Some(mol_dst) = self.atom_mapping[qry_dst] {
            return if let Some(mol_bond) = self.molecule.get_bond(mol_src, mol_dst) {
                if eval_bond_expr(&node.data, mol_bond) {
                    self.match_recursive(op_index + 1)
                } else {
                    false
                }
            } else {
                false
            };
        }

        // ── EXISTING: dst not yet placed, try all neighbours ──
        for bond_idx in 0..self.molecule.bonds.len() {
            let mol_bond = &self.molecule.bonds[bond_idx];
            let other = if mol_bond.source == mol_src {
                mol_bond.dest
            } else if mol_bond.dest == mol_src {
                mol_bond.source
            } else {
                continue;
            };

            if self.atom_mapping.iter().any(|&m| m == Some(other)) {
                continue;
            }

            if eval_bond_expr(&node.data, mol_bond)
                && eval_atom_expr(
                    &self.pattern.nodes[qry_dst].data,
                    &self.molecule.atoms[other],
                )
            {
                self.atom_mapping[qry_dst] = Some(other);
                self.bond_mapping[op_index] = Some(bond_idx);
                if self.match_recursive(op_index + 1) {
                    return true;
                }
                self.atom_mapping[qry_dst] = None;
                self.bond_mapping[op_index] = None;
            }
        }
        false
    }

    fn close_ring(&mut self, op_index: usize) -> bool {
        let node = &self.pattern.nodes[op_index];
        let (Some(mol_src), Some(mol_dst)) = (
            self.atom_mapping[node.src],
            self.atom_mapping[node.dst.unwrap_or(0)],
        ) else {
            return false;
        };

        if let Some(mol_bond) = self.molecule.get_bond(mol_src, mol_dst) {
            if eval_bond_expr(&node.data, mol_bond) {
                let bond_pos = self
                    .molecule
                    .bonds
                    .iter()
                    .position(|b| b == mol_bond)
                    .unwrap();
                self.bond_mapping[op_index] = Some(bond_pos);
                if self.match_recursive(op_index + 1) {
                    return true;
                }
                self.bond_mapping[op_index] = None;
            }
        }
        false
    }
}
