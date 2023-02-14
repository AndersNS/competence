[Lenke til appen](https://variant-kompetanse.pages.dev/)

# Om 

Applikasjonen her er et forsøk på å digitalisere en kompetansekartlegging vi kjørte i Variant Trondheim i februar 2022. 

Applikasjonen er en rust webassembly-applikasjon, grafen er laget med chartjs. 

Akkurat nå så lagres kompetansene man velger i localstorage, man kan velge å lagre det til [KV i CloudFlare via en CF Worker](https://github.com/AndersNS/competence-worker).

Listen over kompetanser er i en [json fil](./example.json).

# Kjøring

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

# Todo

- [ ] CSV-export
- [ ] Håndtere at treet har endret seg på en bedre måte (bare discarde det som ligger i localstorage kanskje?)
- [ ] Bedre måte å navigere mellom "områdene" på (annet enn å scrolle)
- [ ] UI for å lage custom urls
