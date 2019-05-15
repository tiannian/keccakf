# keccakf

An implementation of the keccak-F[1600,800,400,200]. 

## Building

```bash
cargo build
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
keccakf = "0.1.3"
```

and this to your crate root:

```rust
extern crate keccakf;
```

choose which function you want to use:

```rust
use keccakf::KeccakF1600State; // Optional
use keccakf::Permutation;
//...
let state = [0u64;25];
state.permute();
```

## Changelog
- 0.1.3 - Add bits and nbytes in parameter. Change trait `Permutable` to `Permutation`.


