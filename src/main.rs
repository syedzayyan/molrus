// https://payasr.github.io/Are%20Graphs%20hard%20in%20Rust.pdf
// https://efficientbits.blogspot.com/2012/12/the-right-representation-for-job.html
// https://stackoverflow.com/questions/10144394/hashmap-to-implement-adjacency-lists
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]

use parsers::smiles::smiles::parse_smiles;

mod core;
mod parsers;
mod drawing;
mod fingerprints;

fn main (){
    println!("Hello World");
    let mol = parse_smiles("C1=NC(=C2C(=N1)N(C=N2)[C@H]3[C@@H]([C@@H]([C@H](O3)CO)O)O)N").unwrap();

    println!("{:?}", mol.atoms.len())
}