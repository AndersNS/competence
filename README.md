[![Netlify Status](https://api.netlify.com/api/v1/badges/9ef5bffc-b084-4f52-be40-b409332664fb/deploy-status)](https://app.netlify.com/sites/variant-kompetanse/deploys)

[Link to app](https://variant-kompetanse.netlify.com)

# About 

Applikasjonen her er et forsøk på å digitalisere en kompetansekartlegging vi kjørte i Variant Trondheim i februar 2022. 

Applikasjonen er en rust webassembly-applikasjon, grafen er laget med chartjs. 

Akkurat nå så lagres kompetansene man velger i localstorage, og lisen over kompetansene er i en [json fil](./example.json).

# Running

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
