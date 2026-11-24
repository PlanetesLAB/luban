# luban

A spectral PDE framework in Rust, after [Dedalus](https://github.com/DedalusProject/dedalus).

Equations are written in [Typst](https://typst.app) math syntax and checked when the crate
compiles:

```rust
let eq = luban::eq!("partial_t b - kappa nabla^2 b + lift(tau_1) = -u dot nabla b");
```

Terms on the left-hand side are treated implicitly and terms on the right-hand side explicitly.

## Building

```
cargo test --workspace
```

## Licence

MIT. See [LICENSE](LICENSE).
