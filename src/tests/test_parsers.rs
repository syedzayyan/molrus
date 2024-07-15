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
    println!("Reaching Here");
    let smarts_1 = SmartsPattern::new("[12*]");

    assert!(smarts_1.match_mol(&parse_smiles("C").unwrap()));
    assert!(smarts_1.match_mol(&parse_smiles("[CH4]").unwrap()));
    assert!(smarts_1.match_mol(&parse_smiles("[12CH4]").unwrap()));
    assert!(smarts_1.match_mol(&parse_smiles("[13CH4]").unwrap()));

    let smarts_2 = SmartsPattern::new("[13*]");

    assert!(!smarts_2.match_mol(&parse_smiles("C").unwrap()));
    assert!(!smarts_2.match_mol(&parse_smiles("[CH4]").unwrap()));
    assert!(!smarts_2.match_mol(&parse_smiles("[12CH4]").unwrap()));
    assert!(smarts_2.match_mol(&parse_smiles("[13CH4]").unwrap()));

    let smarts_3 = SmartsPattern::new("[0*]");

    assert!(smarts_3.match_mol(&parse_smiles("C").unwrap()));
    assert!(smarts_3.match_mol(&parse_smiles("[CH4]").unwrap()));
    assert!(!smarts_3.match_mol(&parse_smiles("[12CH4]").unwrap()));
    assert!(!smarts_3.match_mol(&parse_smiles("[13CH4]").unwrap()));
}

// #[test]
// fn test_components() {
//     assert!(SmartsPattern::new("(O).(O)".to_string()).match_smarts(&parse_smiles("O.O").unwrap()));
//     assert!(!SmartsPattern::new("(O).(O)".to_string()).match_smarts(&parse_smiles("OO").unwrap()));
// }

// #[test]
// fn test_stereochemistry() {
//     assert!(SmartsPattern::new("C[C@H](O)CC".to_string()).match_smarts(&parse_smiles("C[C@H](O)CC").unwrap()));
//     assert!(!SmartsPattern::new("C[C@H](O)CC".to_string()).match_smarts(&parse_smiles("C[C@@H](O)CC").unwrap()));
//     assert!(!SmartsPattern::new("C[C@H](O)CC".to_string()).match_smarts(&parse_smiles("CC(O)CC").unwrap()));
// }