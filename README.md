# 🌿 rsbonsai

`rsbonsai` is a minimalist, procedurally generated ASCII bonsai tree creator written in Rust. It employs a rule-based agent growth algorithm to simulate the organic development of a tree within a terminal grid. By balancing randomness and structured growth rules, it produces unique, zen-like ASCII art compositions every time it's run.

## ✨ Features

- **Procedural Growth**: Each tree is generated using an agent-based simulation, creating a unique structure every time.
- **Live Animation**: Watch your bonsai grow in real-time with the `--live` mode.
- **Randomized Pots**: A collection of aesthetic ASCII pots are randomly selected to house your tree.
- **Customizable**: Adjust the tree's height, growth speed, and add personal messages.
- **Pure ASCII**: No heavy TUI libraries—just clean, standard terminal output.

## 🚀 Installation

Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

```bash
git clone https://github.com/yourusername/rsbonsai.git
cd rsbonsai
cargo build --release
```

## 🛠 Usage

Run the program using `cargo run` or the compiled binary.

### CLI Arguments

| Flag | Long Flag | Default | Description |
| :--- | :--- | :--- | :--- |
| `-l` | `--live` | `false` | Grow the tree step-by-step with animation. |
| `-d` | `--delay` | `50` | Animation delay in milliseconds (only for `--live`). |
| `-L` | `--life` | `25` | Initial life/height capacity of the main trunk. |
| `-m` | `--message` | `None` | Custom text to display centered below the pot. |

### Examples

**Basic Generation:**
```bash
cargo run
```

**Tall Tree with a Message:**
```bash
cargo run -- --life 40 --message "Zen Garden"
```

**Slow-Motion Growth Animation:**
```bash
cargo run -- --live --delay 100 --life 30 --message "Nature in Code"
```

## 🧠 How it Works

The generator uses a simple yet effective **Agent-Based System**:

1. **The Trunk**: A `BranchAgent` starts at the center of a randomly selected pot and moves upward. It has a chance to "bend" slightly at each step to simulate natural growth.
2. **Branching**: As the trunk grows, it has a probabilistic chance to spawn side branches. These side branches can also spawn smaller sub-branches, creating a fractal-like canopy.
3. **Foliage**: When an agent's "life" expires, it triggers a `draw_leaves` function that clusters random leaf characters (`*`, `&`, `o`, `` ` ``) around its final position.
4. **Rendering**: The simulation operates on a 2D grid (`Canvas`), which is flushed to the terminal. In live mode, ANSI escape codes are used to reset the cursor, creating a seamless animation effect.

## 📜 License

This project is open-source and available under the MIT License.
