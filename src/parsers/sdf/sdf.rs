use std::{fs::File, io::{self, BufRead, BufReader}};

// Honestly Just Copied from CDK
use crate::core::{defs::{Atom, Axialness, Bond}, molecule::Molecule};
use std::str::FromStr;


// Just ChatGPT Code with minimal Validation. 
pub fn read_sdf(file_path: &str) -> io::Result<Vec<Molecule>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut molecules = Vec::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line?;
        if line.trim() == "$$$$" {
            continue;
        }

        let mut molecule = Molecule::new();
        if let Some(line) = lines.next() { line?; } // Skip line 2
        // if let Some(line) = lines.next()

        if let Some(line) = lines.next() {
            let line = line?;
            let num_atoms = usize::from_str(&line[0..3].trim()).unwrap_or(0);
            let num_bonds = usize::from_str(&line[3..6].trim()).unwrap_or(0);

            for _ in 0..num_atoms {
                if let Some(line) = lines.next() {
                    let line = line?;
                    let x = f64::from_str(&line[0..10].trim()).unwrap_or(0.0);
                    let y = f64::from_str(&line[10..20].trim()).unwrap_or(0.0);
                    let z = f64::from_str(&line[20..30].trim()).unwrap_or(0.0);
                    let element = line[31..34].trim().to_string();
                    let mass_diff = usize::from_str(&line[34..36].trim()).unwrap_or(0);
                    let charge = i8::from_str(&line[36..39].trim()).unwrap_or(0);

                    let atom = Atom {
                        outgoing_bond: Vec::new(),
                        element: element.parse().unwrap_or(0),
                        isotope: mass_diff,
                        hydrogens: 0,
                        aromatic: false,
                        f_charge: charge,
                        configuration: None,
                        ring: false,
                        symmetry_class: 0,
                        coords_3d: Some((x, y, z)),
                    };

                    molecule.atoms.push(atom);
                }
            }

            for _ in 0..num_bonds {
                if let Some(line) = lines.next() {
                    let line = line?;
                    let atom1 = usize::from_str(&line[0..3].trim()).unwrap_or(0) - 1;
                    let atom2 = usize::from_str(&line[3..6].trim()).unwrap_or(0) - 1;
                    let bond_order = i8::from_str(&line[6..9].trim()).unwrap_or(0);

                    let bond = Bond {
                        source: atom1,
                        dest: atom2,
                        arom: false,
                        ring: false,
                        bond_order,
                        axialness: Axialness::UNKNOWN,
                    };

                    molecule.bonds.push(bond);
                }
            }

            // Parse the properties block
            while let Some(line) = lines.next() {
                let line = line?;
                if line.trim() == "M  END" {
                    break;
                }

                if line.starts_with("> ") {
                    if let Some(_property_name) = line.split('<').nth(1).and_then(|s| s.split('>').next()) {
                        let mut property_value = String::new();
                        while let Some(line) = lines.next() {
                            let line = line?;
                            if line.trim().is_empty() {
                                break;
                            }
                            property_value.push_str(&line);
                        }
                    }
                }
            }
        }
        molecules.push(molecule);
    }
    Ok(molecules)
}