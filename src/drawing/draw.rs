// use std::fs::File;
// use std::io::Write;
// use crate::core::{defs::Atom, bonds::Bond, molecule::Molecule};

// const SVG_HEADER: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1">"#;
// const SVG_FOOTER: &str = r#"</svg>"#;

// fn atom_to_svg(atom: &Atom, index: usize) -> String {
//     format!(
//         r#"<circle cx="{x}" cy="{y}" r="5" fill="black" id="atom-{index}" />"#,
//         x = index * 20 + 10, 
//         y = 20,
//         index = index
//     )
// }

// fn bond_to_svg(bond: &Bond, source_index: usize, target_index: usize) -> String {
//     format!(
//         r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="black" />"#,
//         x1 = source_index * 20 + 10,
//         y1 = 20,
//         x2 = target_index * 20 + 10,
//         y2 = 20
//     )
// }

// pub fn draw_molecule(molecule: &Molecule) -> String {
//     let mut svg_content = String::new();
//     svg_content.push_str(SVG_HEADER);
    
//     for (index, atom) in molecule.atoms.iter().enumerate() {
//         svg_content.push_str(&atom_to_svg(atom, index));
//     }
    
//     for bond in &molecule.bonds {
//         svg_content.push_str(&bond_to_svg(
//             bond,
//             molecule.bonds.iter().position(|b| b == bond).unwrap(),
//             bond.target
//         ));
//     }
    
//     svg_content.push_str(SVG_FOOTER);
//     svg_content
// }

// pub fn save_svg_to_file(molecule: &Molecule, filename: &str) -> std::io::Result<()> {
//     let svg_content = draw_molecule(molecule);
//     let mut file = File::create(filename)?;
//     file.write_all(svg_content.as_bytes())?;
//     Ok(())
// }