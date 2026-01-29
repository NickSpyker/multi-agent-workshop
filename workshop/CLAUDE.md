# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

```bash
# Build all projects
cargo build

# Run a specific simulation
cargo run --bin game-of-life
cargo run --bin boids
cargo run --bin fluid
cargo run --bin physarum

# Optimized release build
cargo run --bin <name> --release

# Run tests (when added)
cargo test

# Check code without building
cargo check
```

## Code Quality Requirements

**Critical lint rules enforced at workspace level:**
- `unsafe_code = "forbid"` - No unsafe code allowed
- `unwrap_used = "deny"` - Never use `.unwrap()`, use proper error handling
- `expect_used = "deny"` - Never use `.expect()`, use proper error handling
- `panic = "deny"` - Never use `panic!()`, return `Result` types instead
- `clippy::all = "deny"` - All clippy warnings are errors

Always use `Result` types and proper error propagation with `?` operator.

## Architecture Overview

This is a multi-agent simulation workshop with 4 projects: Game of Life, Boids, Fluid, and Physarum. All projects share an identical structure and use the `multi-agent` framework crate.

### Framework Pattern

Each project implements two traits from the `multi-agent` crate:

1. **MultiAgentSimulation** (in `simulation/simulator.rs`):
   - `new(initial_gui_data)` - Initialize simulator from config
   - `update(gui_data, messages, delta_time, send_message_to_gui)` - Run simulation tick

2. **MultiAgentGui** (in `gui/app.rs`):
   - `sidebar(...)` - Render controls, return updated config
   - `content(...)` - Render simulation visualization
   - `received_messages_from_simulation(...)` - Handle simulator messages

3. **AppLauncher** (in `main.rs`):
   ```rust
   AppLauncher::run::<SimulatorType, GuiType>()
   ```

### Message-Passing Architecture

GUI and Simulator communicate via typed enums:
- `MessageFromGuiToSimulator` - User actions (spawn cells, change parameters)
- `MessageFromSimulatorToGui` - Simulation events (statistics, state changes)

### Project File Structure

Each project follows this structure:
```
project-name/src/
├── main.rs                     # AppLauncher bootstrap
├── gui/
│   ├── mod.rs                  # Re-exports
│   ├── app.rs                  # MultiAgentGui implementation
│   ├── data.rs                 # Config structs (paused, tick_rate, etc.)
│   └── message.rs              # MessageFromGuiToSimulator enum
└── simulation/
    ├── mod.rs                  # Re-exports
    ├── simulator.rs            # MultiAgentSimulation implementation
    ├── data.rs                 # Simulation state (grid, particles, etc.)
    └── message.rs              # MessageFromSimulatorToGui enum
```

## Performance Considerations

- **Delta-time independence**: Use `delta_time.as_secs_f32()` for physics calculations
- **Clone efficiency**: Simulation data is cloned ~30x/sec for rendering; use `Arc` for large immutable data
- **Parallelization**: `rayon` is available for CPU-intensive operations (see game-of-life's neighbor counting)
- Config and data structs must implement `Clone`, `Debug`, `Default`

## Current Project Status

- **game-of-life**: Core simulation logic implemented (sparse HashSet grid, Conway's rules), GUI rendering needs completion
- **boids**, **fluid**, **physarum**: Skeleton structure only, ready for implementation
