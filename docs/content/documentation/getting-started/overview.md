+++
title = "Overview"
weight = 5
+++

## Molrus at a Glance

I have no idea how to start this, so read in a molecules first and do simple SMARTS match!

```Rust
// Import Stuff
use molrus::parsers::daylight::{smarts_defs::SmartsPattern, smiles::parse_smiles};

fn main(){
    // Read in a molecule
    let mol = parse_smiles("CCO").unwrap();
    // Read in a SMARTS string
    let smarts_pattern = SmartsPattern::new("[12*]")?;

    // Match your molecule!
    let is_match = smarts_pattern.match_mol(&mol);
    println!("{:?}", is_match)
}
```