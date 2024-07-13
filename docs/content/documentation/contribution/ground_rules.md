+++
title = "Ground Rules"
weight = 6
+++

## Ground Rules

- I have tried to keep the code fairly dependency free. Please try to keep that in mind. Why? Because libraries die. 
- I tend to avoid smart pointers so Arc, RefRc is a bit out of line. I just don't like them having worked with them in RDKit. Yes, I know this is Rust but still. If you absolutely need it because you want multithreaded magic and you are a god-tier programmer, please go ahead.
- Upon download, `cargo build` should do enough to build the codebase. And then the usual stuff you do while doing a PR. Fork the library, make changes and merge into the main on this repo.

### Where do I start?

- Good place to start would be docs. Write like a human being please. Dry, self-deprecating humour is always welcome. Just don't take a dig at me, I have a massive ego that gets hurt easily. 
- Make, tutorials? Those are fine too in the quickstart guide. The world is the limit!
- Implementing new stuff. Okay, ML/AI is frankly out of line, but anything else in cheminformatics is really fine. Don't leak memory, be recursive yadi yadi yada. Also document and stuff please.

### YoUr cOde Is NoT IdIOmaTic RuSt!

Do a PR? It's not like I am claiming to be a Rust or programming God!