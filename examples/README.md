# Multi-Agent Examples

This directory contains example implementations demonstrating the multi-agent framework capabilities.

## Bouncing Balls

A physics simulation showcasing the multi-agent system architecture with interactive bouncing balls.

![Bouncing Balls Example](bouncing-balls/img/bouncing-balls-example.gif)

### Features

- **Physics Simulation**: Realistic gravity and collision dynamics.
- **Interactive Controls**: Add/remove balls, pause/resume, and shake the simulation.
- **Performance**: Efficient agent-based architecture handling multiple entities.
- **Visual Feedback**: Real-time rendering with colorful animated balls.

### How It Works

The simulation implements a multi-agent system where each ball is an independent agent following simple physics rules:

- **Gravity**: Constant downward acceleration (9.81 m/s²).
- **Bouncing**: Energy loss on collision with boundaries (90% damping coefficient).
- **Movement**: Position updates based on velocity and delta time.
- **Collision Detection**: Boundary checking and response.

### Running the Example

```bash
cargo run --example bouncing-balls
```

### Architecture

The example demonstrates the framework's separation of concerns:

- **Simulator** (`simulation/`): Physics engine and ball state management.
- **GUI** (`gui/`): User interface, controls, and rendering.
- **Communication**: Message-passing between GUI and simulation threads.

This example serves as a template for building your own multi-agent simulations with physics, biology, or other emergent behaviors.
