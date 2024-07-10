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