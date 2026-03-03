#[cfg(test)]
mod tests {
    use crate::core::{
        defs::{Atom, Axialness, Bond},
        molecule::Molecule,
    };
    use crate::fingerprints::ecfp::ecfp; // ← this is all you need

    // Helper to build a simple alkane: C–C
    fn build_ethane_like() -> Molecule {
        let mut mol = Molecule::new();

        // Two carbon atoms
        let c1 = Atom {
            element: 6,
            outgoing_bond: Vec::new(),
            isotope: 0,
            hydrogens: 3,
            aromatic: false,
            f_charge: 0,
            configuration: None,
            ring: false,
            symmetry_class: 0,
            coords_3d: None,
        };
        mol.add_atom(c1);

        let c2 = Atom {
            element: 6,
            outgoing_bond: Vec::new(),
            isotope: 0,
            hydrogens: 3,
            aromatic: false,
            f_charge: 0,
            configuration: None,
            ring: false,
            symmetry_class: 0,
            coords_3d: None,
        };
        mol.add_atom(c2);

        // Single bond between them
        mol.add_bond(Bond {
            source: 0,
            dest: 1,
            arom: false,
            ring: false,
            bond_order: 1,
            axialness: Axialness::UNKNOWN,
        });

        mol
    }

    // Helper to build a simple benzene‑like ring
    fn build_benzene_like() -> Molecule {
        let mut mol = Molecule::new();

        // 6 aromatic carbons
        for _ in 0..6 {
            mol.add_atom(Atom {
                element: 6,
                outgoing_bond: Vec::new(),
                isotope: 0,
                hydrogens: 1,
                aromatic: true,
                f_charge: 0,
                configuration: None,
                ring: true,
                symmetry_class: 0,
                coords_3d: None,
            });
        }

        // Ring bonds: 1–2, 2–3, 3–4, 4–5, 5–6, 6–1
        for i in 0..6 {
            mol.add_bond(Bond {
                source: i,
                dest: (i + 1) % 6,
                arom: true,
                ring: true,
                bond_order: 1, // stored as aromatic 1; kekulization is separate
                axialness: Axialness::UNKNOWN,
            });
        }

        mol
    }

    #[test]
    fn test_ecfp_non_empty() {
        let ethane = build_ethane_like();
        let fp = ecfp(&ethane, 2);

        assert!(!fp.is_empty(), "ECFP fingerprint should not be empty");
    }

    #[test]
    fn test_ecfp_different_molecules() {
        let ethane = build_ethane_like();
        let benzene = build_benzene_like();

        let fp1 = ecfp(&ethane, 2);
        let fp2 = ecfp(&benzene, 2);

        assert!(!fp1.is_empty(), "Ethane fingerprint should not be empty");
        assert!(!fp2.is_empty(), "Benzene fingerprint should not be empty");

        // Whole‑set equality is too strict if you later add randomization,
        // but for now we expect different sets.
        assert_ne!(
            fp1, fp2,
            "Ethane and benzene should have different ECFP sets"
        );
    }

    #[test]
    fn test_ecfp_repeatable() {
        let mol = build_ethane_like();

        let fp1 = ecfp(&mol, 2);
        let fp2 = ecfp(&mol, 2);

        assert_eq!(
            fp1, fp2,
            "ECFP on the same molecule should be bit‑identical"
        );
    }
}
