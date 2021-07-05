# ![banner](https://raw.githubusercontent.com/EladJosef/BlueHood/d476f39e7a041707c72a84897582db6e1fa318f1/graphic%20package/xl-banner.svg)

## In the nutshell
BlueHood convert any file type into one encrypted image using Advanced Encryption Standard 256bit (AKA AES).
## Built With
- [Rust](https://www.rust-lang.org/) - used for build main algorithm.
- [Electron](https://www.electronjs.org/) - used for build cross-platform desktop apps (BlueHood app).
- [WebAssembly](https://webassembly.org/) - help us use rust algorithm in BlueHood app.
- [Svelte](https://svelte.dev/) - framework for BlueHood app.
## Build
### CLI app
```bash
cd CLI app
cargo build
```
### WebAssembly
```bash
cd CLI app
wasm-pack build --target web
```
### GUI app
```bash
cd BlueHood app
npm run build
```

## Usge
### CLI app
```bash
git clone https://github.com/EladJosef/BlueHood
cd BlueHood
cd CLI app
cargo build
cargo run [arguments]
```
or just download the exe file from this [link](https://github.com/EladJosef/BlueHood/releases/download/0.0.2v/algorithm.exe)
```bash
app.exe [arguments]
```
### GUI app
```bash
git clone https://github.com/EladJosef/BlueHood
cd BlueHood
cd BlueHood app
npm i
npm run dev
```
## What make it tick?
#### Two main algorithms, encryption and decryption.
<p float="left">
<img src="https://raw.githubusercontent.com/EladJosef/BlueHood/71c0eac017a1a9ef8c76029e9dc646a983c8416e/Program%20design/EncryptAlgorithm.svg" alt="encrypt algorithm design image" width="300"/>
<img src="https://raw.githubusercontent.com/EladJosef/BlueHood/9eb2887c19995bd5af8664ab06a22d4796cc729e/Program%20design/%E2%80%8F%E2%80%8FDecryptAlgorithm.svg" alt="decrypt algorithm design image" width="300"/>
</p>

## Version tracking
#### version number : 0.9.9v
### Next version (0.9.9v) todo list :
 - [ ] Connect wasm to app
