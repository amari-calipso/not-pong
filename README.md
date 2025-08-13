# !pong
What would Pong look like in a parallel universe? !pong ("not pong") lets you jump into that parallel universe, filled with cheap rockets and spacey rocks: see how far you can get without bumping into something! hit the pads as they move, dodge the obstacles, and maybe get some help from magic rainbow crystals scattered in space. 

The original concept is heavily inspired by https://www.lessmilk.com/almost-pong/

# Usage
Left click / touch on right side of screen: jump.
Right click / touch on left side of screen: sprint.

If you just want to play the game, head over to [releases](), or try it online at [WIP]()!

To run or build from source, clone the repository first, then you have a few options:
- To run directly, just use `cargo run`.
- To run in the browser, run:
    - Unix-like: `rustc dev_util.rs -o dev_util && ./dev_util --run-wasm`
    - Windows: `rustc dev_util.rs -o dev_util.exe && dev_util --run-wasm`
- To package a release, (a dist folder containing the release will be created, note that any previously existing dist folders will be deleted): 
    - If releasing for desktop platforms:
        - Unix-like: `rustc dev_util.rs -o dev_util && ./dev_util --release`
            - Windows: `rustc dev_util.rs -o dev_util.exe && dev_util --release`
    - If releasing for web:
        - Unix-like: `rustc dev_util.rs -o dev_util && ./dev_util --release-wasm`
        - Windows: `rustc dev_util.rs -o dev_util.exe && dev_util --release-wasm`

# Screenshots
![](screenshots/start.png)
![](screenshots/many-obstacles.png)