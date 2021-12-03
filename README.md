# 🐦 Shorelark

Simulation of evolution, powered by neural networks, genetic algorithms & high-school math:

Inspired by this Jedi master blog: [Learning to Fly](https://pwy.io/en/posts/learning-to-fly-pt1).

# Building
## Using Cargo and npm

```bash
# 1/3: Clone the repository
$ git clone https://github.com/patryk27/shorelark
$ cd shorelark

# 2/3: Compile Rust into WebAssembly
#      (this might take a minute)
$ cd libs/simulation-wasm
$ wasm-pack build --release

# 3/3: Start the frontend application
$ cd ../../www
$ npm install
$ npm run start

# ^ After launching this, open `http://localhost:8080` in
#   your web browser - you should see the simulation working
#   as the doctor ordered :-)
```