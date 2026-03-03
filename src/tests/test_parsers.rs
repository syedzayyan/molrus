#[cfg(test)]
use crate::parsers::daylight::smiles::parse_smiles;

#[test]
fn test_smiles_parsing() {
    let mol = parse_smiles("C1=NC(=C2C(=N1)N(C=N2)[C@H]3[C@@H]([C@@H]([C@H](O3)CO)O)O)N");
    match mol {
        Ok(mol) => {
            assert_eq!(mol.atoms.len(), 19)
        }
        Err(e) => {
            assert!(false, "Failed to parse SMILES string: {:?}", e);
        }
    }
}

// Stolen SMARTS test from CDK
#[cfg(test)]
use crate::parsers::daylight::smarts_defs::SmartsPattern;
#[test]
fn test_isotopes() {
    let smarts_any = SmartsPattern::new("[*]");
    // true wildcard: matches everything
    assert!(smarts_any.match_mol(&parse_smiles("C").unwrap()));
    assert!(smarts_any.match_mol(&parse_smiles("[12CH4]").unwrap()));
    assert!(smarts_any.match_mol(&parse_smiles("[13CH4]").unwrap()));

    let smarts_12 = SmartsPattern::new("[12*]");
    // isotope 12 only
    assert!(!smarts_12.match_mol(&parse_smiles("C").unwrap())); // isotope=0 → no
    assert!(!smarts_12.match_mol(&parse_smiles("[CH4]").unwrap())); // isotope=0 → no
    assert!(smarts_12.match_mol(&parse_smiles("[12CH4]").unwrap())); // isotope=12 → yes
    assert!(!smarts_12.match_mol(&parse_smiles("[13CH4]").unwrap())); // isotope=13 → no

    let smarts_13 = SmartsPattern::new("[13*]");
    assert!(!smarts_13.match_mol(&parse_smiles("C").unwrap()));
    assert!(!smarts_13.match_mol(&parse_smiles("[CH4]").unwrap()));
    assert!(!smarts_13.match_mol(&parse_smiles("[12CH4]").unwrap()));
    assert!(smarts_13.match_mol(&parse_smiles("[13CH4]").unwrap()));

    let smarts_0 = SmartsPattern::new("[0*]");
    // unspecified isotope only
    assert!(smarts_0.match_mol(&parse_smiles("C").unwrap()));
    assert!(smarts_0.match_mol(&parse_smiles("[CH4]").unwrap()));
    assert!(!smarts_0.match_mol(&parse_smiles("[12CH4]").unwrap()));
    assert!(!smarts_0.match_mol(&parse_smiles("[13CH4]").unwrap()));
}

// ── Component / fragment test ────────────────────────────────────────────────
#[test]
fn test_components() {
    assert!(SmartsPattern::new("[O]").match_mol(&parse_smiles("O").unwrap()));
    assert!(SmartsPattern::new("[O]").match_mol(&parse_smiles("O.O").unwrap()));
    assert!(SmartsPattern::new("[O]").match_mol(&parse_smiles("OO").unwrap()));
}

#[test]
fn test_stereochemistry() {
    // Backbone match without chirality constraint
    let no_stereo = SmartsPattern::new("CC(O)CC");
    assert!(
        no_stereo.match_mol(&parse_smiles("C[C@H](O)CC").unwrap()),
        "backbone should match regardless of chirality"
    );
    assert!(
        no_stereo.match_mol(&parse_smiles("C[C@@H](O)CC").unwrap()),
        "backbone should match regardless of chirality"
    );
    assert!(
        no_stereo.match_mol(&parse_smiles("CC(O)CC").unwrap()),
        "backbone should match unspecified chirality"
    );

    // With chirality flag — AeChiral currently returns true always,
    // so all three match until eval_atom_expr is updated
    let with_stereo = SmartsPattern::new("C[C@H](O)CC");
    assert!(
        with_stereo.match_mol(&parse_smiles("C[C@H](O)CC").unwrap()),
        "same chirality should match"
    );

    assert!(!with_stereo.match_mol(&parse_smiles("C[C@@H](O)CC").unwrap()));
    assert!(!with_stereo.match_mol(&parse_smiles("CC(O)CC").unwrap()));
}
