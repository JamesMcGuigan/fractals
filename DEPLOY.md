
## Deploy

### Web2 Vercel

Autodeployed upon GitHub push
- https://fractals.jamesmcguigan.com/
- https://vercel.com/jamesmcguigan/fractals/
```
BUILD:   source $HOME/.cargo/env; trunk clean; trunk build --release
OUTPUT:  dist
INSTALL: curl https://sh.rustup.rs -sSf | sh -s -- -y; source $HOME/.cargo/env; rustup target add wasm32-unknown-unknown; cargo install trunk; cargo install --path .; 
ROOT:    ./
```

### Web3 Internet Computer
- https://nwb7c-dqaaa-aaaak-qatfa-cai.ic0.app/
- https://nns.ic0.app/#/canisters
- [./canister_ids.json](canister_ids.json)
- [./dfx.json](dfx.json)
```
dfx identity get-principal
dfx deploy --network ic --no-wallet
```
