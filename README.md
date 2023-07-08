# Simple [Bevy](https://bevyengine.org/) and WASM ([WebAssembly](https://webassembly.org/)) Ball Game

# [DEMO](https://volodalexey.github.io/simple-bevy-wasm-ball-game/)

Originally created by [frederickjjoubert](https://github.com/frederickjjoubert) as [Ball Game](https://github.com/frederickjjoubert/learn-bevy) in following video playlist [Learn Bevy Engine 0.10 Beginner Tutorial Series](https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&pp=iAQB).

Refactored to use WASM with touch/mouse support

## Full YouTube playlist:

[Learn Bevy 0.10 - EP1 - Intro Tutorial to Bevy Game Engine Entity Component System and Queries](https://www.youtube.com/watch?v=TQt-v_bFdao)

[Learn Bevy 0.10 - EP2 - First Game Setup + Bundles and Resources - Bevy Tutorial](https://www.youtube.com/watch?v=izhFutJiZgo)

[Learn Bevy 0.10 - EP3 - Player Movement from Keyboard Input and Spawning Enemies - Bevy Tutorial](https://www.youtube.com/watch?v=xnGMw5j5Xdo)

[Learn Bevy 0.10 - EP4 - Enemy Movement, Collisions, Sound Effects, Despawning - Bevy Tutorial](https://www.youtube.com/watch?v=4TjEo-gDgAg)

[Learn Bevy 0.10 - EP5 - Spawning and Collecting Stars, Score Resource, and Timers - Tutorial](https://www.youtube.com/watch?v=zdmZ95l-vzU)

[Learn Bevy 0.10 - EP6 - Spawning Enemies over Time, and Bevy Events! - Bevy Engine Beginner Tutorial](https://www.youtube.com/watch?v=GOl-kacs8TQ)

[Learn Bevy 0.10 - EP7 - Bevy Project Organization and Refactor, Rust Modules, and Bevy Plugins](https://www.youtube.com/watch?v=gy2G63SA-W8)

[Learn Bevy 0.10 - EP8 - Explicit System Ordering and Bevy System Sets](https://www.youtube.com/watch?v=i-Wczghlmxc)

[Learn Bevy 0.10 - EP9 - Bevy States and Run Conditions](https://www.youtube.com/watch?v=bD2rgvtXcq0)

[Learn Bevy 0.10 - EP10 - Bevy UI (User Interface Tutorial)](https://www.youtube.com/watch?v=iW19V3a96tY)

## Compile to WASM (WebAssembly) and build for browser

### Add WebAssembly support to your Rust installation
```sh
rustup target install wasm32-unknown-unknown
```

### Install [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) CLI
```sh
cargo install wasm-bindgen-cli
```
Additionally for optimization wasm bundle size you can install [wasm-opt](https://github.com/brson/wasm-opt-rs) tool
```sh
cargo install wasm-opt
```

### Run build script

```sh
./utils/wasm/build.sh
```
Or build and optimize
```sh
./utils/wasm/optimize.sh
```

## WASM build history

### Initial
- 24603385    simple-bevy-wasm-ball-game_bg.wasm
- 88108       simple-bevy-wasm-ball-game.js

### Replaced rand with fastrand
- 24585077    simple-bevy-wasm-ball-game_bg.wasm
- 88108       simple-bevy-wasm-ball-game.js

### Use 3d assets from glb file and animations
- 24725317    simple-bevy-wasm-ball-game_bg.wasm
- 88108       simple-bevy-wasm-ball-game.js

### Use Instant from bevy
- 24682460    simple-bevy-wasm-ball-game_bg.wasm
- 88108       simple-bevy-wasm-ball-game.js

### Use Optimization (audio initialization do not work)
- 14901228    simple-bevy-wasm-ball-game_bg.wasm
- 88108       simple-bevy-wasm-ball-game.js

### Use Rapier physics engine
- 26932957    simple-bevy-wasm-ball-game_bg.wasm
- 88108       simple-bevy-wasm-ball-game.js

### Use optimizations for modules
- 26990021    simple-bevy-wasm-ball-game_bg.wasm
- 88108       simple-bevy-wasm-ball-game.js