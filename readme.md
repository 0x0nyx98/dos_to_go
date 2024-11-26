# dos support for rust :3
yes its named after hot to go

# /!\ very unfinished it has like two functions rn. let me cook

# how 2 use:
step 1: make your rust project (cargo new blahblahblahwhatever)

step 2: copy the contents of the quickstart folder in this repo into your new project folder (they should be on the same level as src, so .cargo and src should now be two subdirs of the same folder)

step 3: add the dos_to_go dependency (cargo add dos_to_go)

step 4: rename your main.rs to lib.rs and add this header:
```rust
#![no_std]
use core::arch::asm;
use dos_to_go::*;
```

step 5: add these lines to your cargo.toml:
```rust
[lib]
crate-type = ["staticlib"]
```

step 6: make sure your main function returns a u8, and give it the `#[dos_to_go::entrypoint]` attribute

step 7: now, in your terminal, run `nix-shell`

step 8: congration! you can now run `build` to build a dos .exe, and `run` will automatically build and run the app in dosbox! :D
