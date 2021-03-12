# N-D Raytracing Web Frontend

See: [nd-raytracing.netlify.app](https://nd-raytracing.netlify.app/)

## Getting Started

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install), then run:

```sh
npm run serve
```

## SIMD

Ray tracing is something that can tremendously benefit from SIMD.
[WASM support for SIMD](https://github.com/WebAssembly/simd) is still experimental but
you can try it out if you build with the following `RUSTFLAGS`:

```sh
env RUSTFLAGS="-C target-feature=+simd128,+unimplemented-simd128" npm run serve
```

This will turn on auto-vectorization in Rust and emit WASM with SIMD instructions. To try
this out you will need to activate the corresponding flags in your browser or perhaps just
wait for a little. In chrome the flag can be set using `chrome://flags/#enable-webassembly-simd`
