# Running

[![Netlify Status](https://api.netlify.com/api/v1/badges/9ef5bffc-b084-4f52-be40-b409332664fb/deploy-status)](https://app.netlify.com/sites/variant-kompetanse/deploys)

Install trunk 
```bash
cargo install trunk
```

Install webassembly target

```bash
rustup target add wasm32-unknown-unknown
```

Compile and host

```
trunk serve --open
```
