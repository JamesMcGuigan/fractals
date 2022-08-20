
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

Links:
- https://nwb7c-dqaaa-aaaak-qatfa-cai.ic0.app/
- https://nns.ic0.app/#/canisters

Config Files:
- [./canister_ids.json](canister_ids.json)
- [./dfx.json](dfx.json)
```
dfx identity get-principal
dfx deploy --network ic --no-wallet
```

### Vercel IC Proxy
https://fractals.icp.jamesmcguigan.com

- Github Repo:
  - https://github.com/JamesMcGuigan/ic-proxy 
  - https://github.com/dfinity/ic
- Edit Config: 
  - https://github.com/JamesMcGuigan/ic-proxy/edit/master/typescript/service-worker/src/sw/http_request.ts
  - `'fractals.icp.jamesmcguigan.com': ['nwb7c-dqaaa-aaaak-qatfa-cai', 'ic0.app'],`  
- Vercel Config: 
  - https://vercel.com/jamesmcguigan/icp
  - https://vercel.com/jamesmcguigan/icp/settings 
  - https://vercel.com/jamesmcguigan/icp/settings/domains
```
BUILD:   npm run build
OUTPUT:  dist-prod
INSTALL: npm install
DEVELOPMENT:      
```