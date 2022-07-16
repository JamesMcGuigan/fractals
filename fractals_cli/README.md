# Fractals CLI

Moving the CLI to a separate module to avoid conflicts with Yew WASM
```
time cargo run --release --bin julia_image -q

cargo build --release;
./target/release/julia_image -c 0.5+0.5i -r 1 -o fractal.jpg
```
![](./fractal.png)
