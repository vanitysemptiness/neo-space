# neo-space
"In the future, frontends will be written in C, C++, and Rust. Backends will be written in Typescript." - You if contribute to this project

A Rust based open source infinite canvas project that will attempt to have many of the features of figma, and certainly more features than freeform.
This project will work both as WebAssembly compiled from Rust and as a standalone native Rust application. Write and add any plugin you want.
Download, embed, share the code as much as you want. No corporate middle men. 

Eventually there will be a SDK developed allowing for the easy integration of the canvas and various drawing features. 

# Building and Running the Project
local: in project root
```bash
cargo run
```
web assembly
```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir ./pkg --target web ./target/wasm32-unknown-unknown/release/neospace.wasm
```

# helpful git commands for contributing
fork the original repo, download it, and work off a new branch
```bash
git checkout -b name-that-implies-your-change
```
verify that you are tracking the remote
```bash
git remote -v
```
you might be tracking your own remote, add neo-space/neo-space as a remote
```bash
git remote add upstream https://github.com/neo-space/neo-space.git
```
check your `origin` and `upstream`, you should see something like this.
origin is your fork and neo-space is the original
```bash
origin  git@github.com:vanitysemptiness/neo-space.git (fetch)
origin  git@github.com:vanitysemptiness/neo-space.git (push)
upstream        https://github.com/neo-space/neo-space.git (fetch)
upstream        https://github.com/neo-space/neo-space.git (push)
```

fetch latest changes from upstream repository
```bash
git fetch upstream
```
change to your main branch, ditch your changes in main (which should be none as you use branches), and reset to upstream/main
```bash
git checkout main
git reset --hard upstream/main
```
push your updated main to your forks main 
```bash
git push origin main --force
```
branching, making changes, and pushing them to your fork (`origin`)
```bash
git checkout -b my-new-feature
# Make your changes, commit them
git push origin my-new-feature
```
Once thats there, raise the PR against neo-space/neo-space on github

Try your best to follow conventional commits https://www.conventionalcommits.org/en/v1.0.0/
I can't promise that I will only ever be conventional, nor do I promise that I will only accept other's PR's if they are conventional,
but why not try?