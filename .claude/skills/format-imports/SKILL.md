---
name: format-imports
description: Format / reorder imports
---

## Instructions
Locate the `src` directory. Inspect each Rust source code (`.rs`) file. Group
and then reorder the imports and `mod` declarations as follows:

1. All `mod` declarations should come first (if they exist)
2. Standard library imports should come next (if they exist)
3. External crate imports should come next (if they exist)
4. Finally, local crates / submodules should come last (if they exist)

Place a newline between each group you create. Finally, run `cargo fmt` as
well, which will automatically sort the groups.

## Expected output
The output should look something like:
```rust
use std::collections::HashMap;
use std::fs::File;

use clap::Parser;
use rand::Rng;

use crate::models::User;
use super::helpers::validate;

// rest of code
```
