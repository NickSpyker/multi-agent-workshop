# Multi-Agent System & Simulation Workshop

Welcome to the Multi-Agent System & Simulation Workshop!
This workshop introduces you to agent-based modeling through hands-on implementation of classic simulations.

## What You'll Build

Choose one or more of these simulations to implement:

- **Conway's Game of Life:** The foundational cellular automaton.
- **Boids:** Flocking behavior of birds and fish.
- **Physarum:** Slime mold and biological transport networks.
- **Fluid Dynamics:** Physical particle simulation.

## How the Framework Works

The framework provides everything except the simulation logic.
Here's what you need to know:

### The Big Picture

1. **You get a ready-made GUI:** Already connected and configured for your project.
2. **You get data structures:** Input (`Config`) and Output (`Data`) structs are predefined.
3. **You implement one trait:** Just write the `MultiAgentSimulation` trait with the `update()` function.
4. **The framework handles everything else:** Threading, synchronization, rendering, GUI lifecycle.

### Your Job: Implement the Simulation Logic

Each project has this structure:

```txt
your-project/
└── src/
    ├── main.rs             # Already Done: Just launches the app
    ├── simulation/
    │   ├── mod.rs
    │   ├── simulator.rs    # YOUR WORK: Implement update() here
    │   ├── data.rs         # Output:    Your simulation state (add fields here)
    │   └── message.rs      # Optional:  Define messages from simulation to GUI (events/notifications)
    └── gui/
        ├── mod.rs
        ├── app.rs          # Already Done: GUI is connected
        ├── data.rs         # Input:        GUI configuration (add fields here)
        └── message.rs      # Optional:     Define messages from GUI to simulation (commands/interactions)
```

### What You Need to Do

#### Step 1: Define Your Data Structures

**In `simulation/data.rs`** - Define what your simulation tracks:

```rust
#[derive(Debug, Default, Clone)]
pub struct YourSimulation {
    // Add your simulation state here
    // Example: pub agents: Vec<Agent>,
}
```

**In `gui/data.rs`** - Define what parameters the user can control:

```rust
#[derive(Clone)]
pub struct YourConfig {
    // Add configuration parameters here
    // Example: pub agent_count: usize,
}

impl Default for YourConfig {
    fn default() -> Self {
        Self {
            // Add default configuration parameters here
            // Example: agent_count: 500,
        }
    }
}
```

#### Step 2: Implement the Update Function

**In `simulation/simulator.rs`** - This is where your simulation logic goes:

```rust
impl MultiAgentSimulation for YourSimulator {
    // ... type definitions already set up ...

    fn new(initial_gui_data: Self::GuiData) -> Result<Self> {
        // Initialize your simulation with the GUI configuration
        Ok(Self {
            data: YourSimulation::default(),
        })
    }

    fn update<F>(
        &mut self,
        gui_data: Self::GuiData,        // Input from GUI
        messages: Vec<Self::MessageFromGui>,
        delta_time: Duration,            // Time since last update
        send_message_to_gui: F,
    ) -> Result<&Self::SimulationData> {
        // YOUR SIMULATION LOGIC HERE
        // 1. Read gui_data for user parameters
        // 2. Update your agents/cells/particles
        // 3. Return reference to your simulation state

        Ok(&self.data)
    }
}
```

**The `update()` function:**
- Is called **30 times per second** by default
- Receives `gui_data` with current user settings
- Receives `delta_time` for physics calculations
- Must return a reference to your simulation state
- The framework automatically sends your state to the GUI for rendering

#### Step 3: Customize the GUI (Optional)

The GUI is already working, but you can enhance it:

**In `gui/app.rs`** - Add controls in the sidebar:

```rust
fn sidebar<F>(...) -> Option<Self::GuiData> {
    let mut changed = false;

    // Add UI controls here
    if ui.button("Reset").clicked() {
        changed = true;
    }

    // Return new config only if something changed
    if changed {
        Some(Self::GuiData::default())
    } else {
        None
    }
}
```

**In `gui/app.rs`** - Render your simulation in the content area:

```rust
fn content<F>(...) {
    // Access simulation state
    let state = &**simulation_data;

    // Draw your simulation
    // Example: Draw agents, cells, particles, etc.
}
```

## Running Your Simulation

From the workshop directory:

```bash
# Run the simulation
cargo run --bin <your-simulation>

# Build optimized version (faster)
cargo run --bin <your-simulation> --release
```

## Tips for Success

### Keep Data Structures Small

Your simulation state is cloned 30 times per second.
Keep structs lightweight or use `Arc` for large data:

```rust
// Good: Small struct
pub struct GameOfLife {
    pub grid: Vec<Vec<bool>>,  // Reasonable size
}

// Better for large data: Use Arc
pub struct Fluid {
    pub particles: Arc<Vec<Particle>>,  // Shared, not cloned
}
```

### Handle Errors Properly

The framework forbids `unwrap()`, `expect()`, and `panic!()`. Always use `Result`:

```rust
// Don't do this:
let value = some_operation().unwrap();  // ❌ Won't compile

// Do this instead:
let value = some_operation()?;  // ✅ Propagates error
```

### Use Delta Time for Physics

Make your simulation framerate-independent:

```rust
fn update(..., delta_time: Duration, ...) -> Result<&Self::SimulationData> {
    let dt = delta_time.as_secs_f32();

    for agent in &mut self.data.agents {
        agent.position += agent.velocity * dt;  // Smooth motion
    }

    Ok(&self.data)
}
```

### Start Simple, Iterate

1. First, get something visible on screen.
2. Then, add the core behavior.
3. Finally, refine and optimize.

## Project-Specific Guides

Each project directory has its own README with:
- Algorithm explanation.
- Implementation hints.
- Expected behavior.
- References.

## Need Help?

- Check the [`examples/bouncing-balls/`](https://github.com/NickSpyker/multi-agent-workshop/tree/main/examples) for a complete working example.
- Each simulation has its own README with specific guidance.

## Good Luck!

Remember: Multi-agent systems are about **simple rules creating complex behavior**.
Start with basic agent interactions and watch emergence happen!
