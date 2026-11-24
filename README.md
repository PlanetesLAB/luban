# luban

A spectral PDE framework in Rust, after [Dedalus](https://github.com/DedalusProject/dedalus).

Equations are written in [Typst](https://typst.app) math syntax and checked when the crate
compiles:

```rust
let eq = luban::eq!("partial_t b - kappa nabla^2 b + lift(tau_1) = -u dot nabla b");
```

Terms on the left-hand side are treated implicitly and terms on the right-hand side explicitly.

## The name

luban is named after [Lu Ban](https://en.wikipedia.org/wiki/Lu_Ban) (鲁班), also known as Gongshu
Ban, a carpenter and engineer of the state of Lu in the fifth century BCE. Later tradition credits
him with many of the carpenter's basic tools and honours him as the patron of builders and
craftsmen. The *Mozi* ([Lu's Question](https://ctext.org/mozi/lus-question), Chinese Text Project)
records that he carved a bird from bamboo and wood that flew for three days without coming down.

## Building

```
cargo test --workspace
```

## Licence

MIT. See [LICENSE](LICENSE).
