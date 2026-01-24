# game_of_life
A simple implementation of game of life in rust

## Build notes (bundled SDL2 on Linux)
This project uses bundled SDL2 to avoid version mismatches with system SDL.

### First build
```
cargo build
```

### Local run
```
cargo run
```

### Troubleshooting (PipeWire + new toolchains)
If the bundled SDL build fails with PipeWire or C23 errors, this repo ships a
toolchain file that disables PipeWire and forces C11/gnu11:

- `.cargo/config.toml` sets `SDL2_TOOLCHAIN` and `CMAKE_POLICY_VERSION_MINIMUM`
- `cmake/sdl2-toolchain.cmake` disables PipeWire and sets C standard
