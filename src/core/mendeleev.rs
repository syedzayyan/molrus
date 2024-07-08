pub fn valence_electrons(atomic_number: usize) -> usize {
    match atomic_number {
        1 => 1,    // Hydrogen
        2 => 2,    // Helium
        3..=10 => (atomic_number - 2) % 8,  // Lithium to Neon
        11..=18 => (atomic_number - 10) % 8, // Sodium to Argon
        19..=36 => (atomic_number - 18) % 8, // Potassium to Krypton
        37..=54 => (atomic_number - 36) % 8, // Rubidium to Xenon
        55..=86 => (atomic_number - 54) % 8, // Cesium to Radon
        87..=118 => (atomic_number - 86) % 8, // Francium to Oganesson
        _ => 0,  // Default case for atomic numbers out of range
    }
}