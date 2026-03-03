pub fn valence_electrons(atomic_number: usize) -> usize {
    match atomic_number {
        1 => 1,                               // Hydrogen
        2 => 2,                               // Helium
        3..=10 => (atomic_number - 2) % 8,    // Lithium to Neon
        11..=18 => (atomic_number - 10) % 8,  // Sodium to Argon
        19..=36 => (atomic_number - 18) % 8,  // Potassium to Krypton
        37..=54 => (atomic_number - 36) % 8,  // Rubidium to Xenon
        55..=86 => (atomic_number - 54) % 8,  // Cesium to Radon
        87..=118 => (atomic_number - 86) % 8, // Francium to Oganesson
        _ => 0,                               // Default case for atomic numbers out of range
    }
}

pub fn target_valences_for_smiles(z: usize) -> &'static [i8] {
    // Same as above, but keep it in your chem module
    match z {
        5 => &[3],                // B
        6 => &[4],                // C
        7 => &[3, 5],             // N
        8 => &[2],                // O
        15 => &[3, 5],            // P
        16 => &[2, 4, 6],         // S
        9 | 17 | 35 | 53 => &[1], // F, Cl, Br, I
        _ => &[0],
    }
}

pub fn compute_implicit_h_from_valences(z: usize, bond_order_sum: i32) -> usize {
    let targets = target_valences_for_smiles(z);
    let valence = bond_order_sum.max(0);

    let target = targets
        .iter()
        .filter(|&&t| t >= valence as i8)
        .min()
        .copied()
        .unwrap_or(0);

    if target == 0 {
        0
    } else {
        (target - valence as i8).max(0) as usize
    }
}
