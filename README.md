# bevy-force-directed-graph

`bevy-force-directed-graph` visualized graphs using a [force-directed graph
drawing algorithm](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
using the [bevy](https://github.com/bevyengine/bevy) game engine. It simulates
the nodes and edges based on physical forces (i.e. links are springs, nodes
repell each other) and user input (i.e. drag and drop).

This project is heavily inspired by [d3-force](https://d3js.org/d3-force) which
also includes a great explanation about how to simulate a force-directed graph.

Demo: [joholl.github.io/bevy-force-directed-graph](https://joholl.github.io/bevy-force-directed-graph)

## How the Physics Engine Works

The simulation is very simple. All calculation is based on the following data per node:
* position (x: f32, y: f32)
* last position (x: f32, y: f32)
* locked (bool), true if forces/inertia must not move a node since it is dragged by the user

Physical Value|Implementation
-|-
position | Very important, all forces (and inertia) directly modify the position
velocity | We do not save/modify the velocity. Instead, we use use the position of the previous simulation step to approximate the velocity (see [Verlet Integration](https://en.wikipedia.org/wiki/Verlet_integration)). This is needed for inertia.
acceleration | Forces change the position directly. Thus we do not need to save/modify the acceleration which makes it **irrelevant**.
mass | Assumed to be 1 for all nodes and thus **irrelevant**
time step | Assumed to be constant and thus **irrelevant**

## Forces

Ordered by importance:

Physical Effect | Description
-|-
Link force | Models edges as springs with a given target distance.
Repulsion | Applies a repelling force between all nodes (electrical charge).
Inertia | Not a force in a physical sense. Simulates momentum, allowing nodes to continue moving after being acted upon. Velocity decay (friction) helps the simulation to converge.
Mean-to-center | Not a force in a physical sense. Moves all nodes so that their mean is in the center of the screen. Ensures that the screen is used effectively.
Window-border | Not a force in a physical sense. Prevents nodes from moving outside the visible window area.
Galaxy | Sample force. Swirls around the graph by applying a soup-stirring force-field.


## Build & Run

On Ubuntu, to install the dependencies

```sh
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
```

If using Wayland, you will also need to install

```sh
sudo apt-get install libwayland-dev libxkbcommon-dev
```

For WSL2, install

```sh
sudo apt install gucharmap
```

In [Cargo.toml](Cargo.toml), enable/disable wayland as needed.

Now you can run it. To not recompiling bevy unneccessarily, use dynamic linking.

```sh
cargo run --features bevy/dynamic_linking
```

# Wasm

cargo install wasm-bindgen-cli
wasm-bindgen --out-dir examples/wasm/target --out-name wasm_example --target web target/wasm32-unknown-unknown/debug/megascope.wasm
python -m http.server -d examples/wasm
