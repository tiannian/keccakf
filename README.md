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
keccakf = "1.0"
```

and this to your crate root:

```rust
extern crate keccakf;
```

choose which function you want to use:

```rust
use keccakf::KeccakF1600State;
use keccakf::Permutable;
//...
let state = [0u64;25];
state.permute();
```

