[Lenke til appen](https://variant-kompetanse.pages.dev/)

# About 

Applikasjonen her er et forsøk på å digitalisere en kompetansekartlegging vi kjørte i Variant Trondheim i februar 2022. 

Applikasjonen er en rust webassembly-applikasjon, grafen er laget med chartjs. 

Akkurat nå så lagres kompetansene man velger i localstorage, og lisen over kompetansene er i en [json fil](./example.json).

# Running

You need rust installed first. This [getting started guid](https://www.rust-lang.org/learn/get-started) will help you get rust going. 

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
