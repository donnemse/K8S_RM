# KUBE-RM

A terminal UI tool for monitoring Kubernetes cluster resources (CPU, Memory) at a glance.

```
██╗░░██╗██╗░░░██╗██████╗░███████╗░░░░░░██████╗░███╗░░░███╗
██║░██╔╝██║░░░██║██╔══██╗██╔════╝░░░░░░██╔══██╗████╗░████║
█████═╝░██║░░░██║██████╦╝█████╗░░█████╗██████╔╝██╔████╔██║
██╔═██╗░██║░░░██║██╔══██╗██╔══╝░░╚════╝██╔══██╗██║╚██╔╝██║
██║░╚██╗╚██████╔╝██████╦╝███████╗░░░░░░██║░░██║██║░╚═╝░██║
╚═╝░░╚═╝░╚═════╝░╚═════╝░╚══════╝░░░░░░╚═╝░░╚═╝╚═╝░░░░░╚═╝
```

## Features

### 3 View Modes

| View | Columns |
|---|---|
| **Node** | Node Name, CPU/Memory Allocatable, CPU/Memory Request/Limit |
| **Pod** | Namespace, Pod Name, Status, Node, CPU/Memory Request/Limit |
| **Namespace** | Namespace, CPU/Memory Request/Limit |

- Displays a **TOTAL** summary row at the bottom of each view.
- Supports column-based **sorting**.
- Press `Enter` in Node/Namespace view to **drill down** into the filtered Pod list.

## Key Bindings

| Key | Action |
|---|---|
| `↑` / `↓` | Navigate rows |
| `←` / `→` | Change sort column |
| `Tab` | Switch view mode (Node → Pod → Namespace) |
| `Enter` | Drill down from Node/Namespace to Pod |
| `Esc` | Clear filter |
| `Space` | Refresh data |
| `PageUp` / `PageDown` | Page scroll |
| `Ctrl+C` | Quit |

## Prerequisites

- Rust 1.70+
- `~/.kube/config` file (Kubernetes cluster access configuration)

## Build

```bash
cargo build --release
```

### Cross-compile for Linux (musl)

```bash
export OPENSSL_DIR="$(brew --prefix openssl)"
cargo build --release --target=x86_64-unknown-linux-musl
```

Building with [Cross](https://github.com/cross-rs/cross) is also supported:

```bash
cross build --release --target=x86_64-unknown-linux-musl
```

## Run

```bash
./target/release/kube-rm
```

## Tech Stack

- **Rust** (Edition 2021)
- [kube-rs](https://github.com/kube-rs/kube) - Kubernetes API client
- [tui-rs](https://github.com/fdehau/tui-rs) / [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal UI
- [tokio](https://tokio.rs/) - Async runtime

## Project Structure

```
src/
├── main.rs              # Entrypoint, event loop
├── api/
│   ├── node.rs          # Node resource queries
│   ├── pod.rs           # Pod resource queries
│   └── namespace.rs     # Namespace resource queries
├── models/
│   ├── app.rs           # App state and view modes
│   ├── config.rs        # Sort/search configuration
│   ├── error.rs         # Error types
│   └── resource.rs      # Resource value models
├── ui/
│   ├── ui.rs            # UI rendering
│   └── event.rs         # Keyboard event handling
└── util/
    └── common.rs        # CPU/Memory formatting utilities
```

## Developed by

Data Platform Team (dev.dp@igloo.co.kr)
