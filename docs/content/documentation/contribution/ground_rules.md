+++
title = "Ground Rules"
weight = 6
+++

## Ground Rules

- I have tried to keep the code fairly dependency free. Please try to keep that in mind. Why? Because libraries die. 
- I tend to avoid smart pointers so Arc, RefRc is a bit out of line. I just don't like them having worked with them in RDKit. Yes, I know this is Rust but still.
- Upon download, `cargo build` should do enough to build the codebase. And then the usual stuff you do while doing a PR. Fork the library, make changes and merge into the main on this repo.