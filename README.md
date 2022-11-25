# keccakf

An implementation of the keccak-F[1600,800,400,200]. 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
keccakf = "0.2"
```

choose which function you want to use:

```rust
use keccakf::{Keccak1600State, Permutation};
let state = Keccak1600State::default();
state.permute();
```

## Changelog
- 0.2.1: update README
- 0.2.0: refactor code.
- 0.1.3: Add bits and nbytes in parameter. Change trait `Permutable` to `Permutation`.


