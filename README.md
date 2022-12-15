# excalidraw-wasm

## 🚀 Serve Locally
### Dependencies
- [Rust](https://www.rust-lang.org/)
- [trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [wasm32-unkown-unknown](https://yew.rs/docs/getting-started/introduction#install-webassembly-target) (`rustup target add wasm32-unknown-unknown`)
- [tailwindcss](https://tailwindcss.com/) (`yarn global add tailwindcss`)
### Serve
- Run: `trunk serve`
- Preview: [`http://localhost:8080/`](http://localhost:8080/)

### Build
- Run: `trunk build --release`
- Output: `dist/`