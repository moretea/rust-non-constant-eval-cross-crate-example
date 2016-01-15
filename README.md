Cross-crate example for [#30936](https://github.com/rust-lang/rust/issues/30936)

```
cargo build
   Compiling child v0.1.0 (file:///home/maarten/tmp/noez)
   Compiling noez v0.1.0 (file:///home/maarten/tmp/noez)
src/lib.rs:20:7: 20:31 error: constant evaluation error: non-constant path in constant expression [E0080]
src/lib.rs:20   A = child::OtherCrateEnum::A as isize,
                    ^~~~~~~~~~~~~~~~~~~~~~~~
src/lib.rs:20:7: 20:31 help: run `rustc --explain E0080` to see a detailed explanation
src/lib.rs:21:7: 21:31 error: constant evaluation error: non-constant path in constant expression [E0080]
src/lib.rs:21   B = child::OtherCrateEnum::B as isize
                    ^~~~~~~~~~~~~~~~~~~~~~~~
src/lib.rs:21:7: 21:31 help: run `rustc --explain E0080` to see a detailed explanation
error: aborting due to 2 previous errors
Could not compile `noez`.
```
