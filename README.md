# 🤖 Filler

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:CE412B,100:FF4500&height=190&text=%20&fontAlign=50&fontAlignY=32&fontColor=FFFFFF&desc=Algorithmic%20Game%20Bot%20%7C%20Rust%20%2B%20Docker&descAlign=50&descAlignY=42&animation=twinkling" alt="Filler Banner" />
</p>

<p align="center">
  <a href="#-tech-stack">Tech Stack</a> •
  <a href="#-what-makes-filler-special">Highlights</a> •
  <a href="#-architecture--data-flow">Architecture</a> •
  <a href="#-algorithm--game-logic">Algorithm</a> •
  <a href="#-getting-started">Get Started</a>
</p>

<p align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&pause=900&center=true&vCenter=true&width=980&lines=Competitive+algorithmic+bot+written+in+Rust;Dominate+the+Anfield+grid+against+challenging+opponents;Powered+by+a+greedy+distance-based+strategy;Dockerized+environment+ready+for+testing" alt="Typing SVG" />
</p>

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=rect&color=0:FF4500,100:111827&height=4&section=footer" width="100%" alt="Divider" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Standard_Library-CE412B?style=flat&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Docker-Ready-2496ED?style=flat&logo=docker" alt="Docker" />
  <img src="https://img.shields.io/badge/Algorithm-Greedy-success" alt="Algorithm" />
  <img src="https://img.shields.io/badge/Status-Competitive-blueviolet" alt="Status" />
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/devicons/devicon/master/icons/rust/rust-original.svg" width="34"/>
  <img src="https://raw.githubusercontent.com/devicons/devicon/master/icons/docker/docker-original.svg" width="34"/>
</p>

---

Filler is an algorithmic game where two bots compete against each other to dominate a predefined grid, known as the **Anfield**. The objective is straightforward but challenging: cover as much area as possible. The game engine drops randomly sized and shaped pieces turn-by-turn. The player who successfully places more of their pieces and restricts the opponent's movements wins the game.

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=rect&color=0:FF4500,100:111827&height=4&section=footer" width="100%" alt="Divider" />
</p>

## ⭐ Key Highlights

- **Competitive AI Core:** Engineered with a highly competitive greedy distance-based algorithm.
- **Strict Validity Enforcement:** Mathematical boundary and territory overlay validation (Rule of One, Rule of Zero).
- **Center of Mass Calculation:** Dynamically calculates point of convergence to snuff out enemy territory dynamically.
- **Robust Parsing:** Continuous STDIN buffer consumption specifically designed for dynamic grid and piece dimension structures.
- **Battle-Ready Environment:** Fully Dockerized VM environment seamlessly resolving architecture mismatch constraints.

<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=rect&color=0:FF4500,100:111827&height=4&section=footer" width="100%" alt="Divider" />
</p>

## 📋 Table of Contents

- [✨ What Makes Filler Special](#-what-makes-filler-special)
- [🛠️ Tech Stack](#-tech-stack)
- [🏗️ Architecture & Data Flow](#-architecture--data-flow)
- [🧠 Algorithm & Game Logic](#-algorithm--game-logic)
- [🚀 Getting Started](#-getting-started)
- [💻 Technical Documentation](#-technical-documentation)
- [🐳 Docker Setup & Automation](#-docker-setup--automation)

<details open>
<summary><strong>✨ What Makes Filler Special (expand/collapse)</strong></summary>

## ✨ What Makes Filler Special

### 🎯 The "Move Toward Opponent" Strategy
Instead of blindly picking the first valid placement, Filler implements an aggressive "greedy" strategy. The bot re-calculates the "Center of Mass" of the opponent's total territory during every turn, acting as a heat-seeking system. 

### 🧮 Euclidean Distance Predictor
Our bot runs simulations for every possible placement over the coordinate plane. It utilizes Euclidean distance formulas `sqrt(dx^2 + dy^2)` to guarantee that every piece placed directly intersects with the opponent's growth trajectories, cutting off their "playable oxygen" efficiently.

### 🛡️ Iron-clad Overlap Regulations
The placement engine leverages brute-force arrays expanding natively to negative axis bounds (`-(piece_size - 1)`). It successfully enforces:
- **Condition 1**: Exactly 1 solid overlay onto allied territory.
- **Condition 2**: Exactly 0 intersections over enemy boundaries.
- **Condition 3**: 0 solid parts protruding out of bounds.

</details>

---
## 🛠️ Tech Stack

What powers the Filler Bot locally and in execution simulations.

### The Bot Engine
- **Rust (Standard Library)** - Highly performant, statically typed execution required for timed environments.

### DevOps - Getting It Running reliably
- **Docker** - Same environment everywhere, regardless of OS variations.
- **Linux Game Engine VM** - Orchestrates the battlefield rules and streams stdout reliably.

---
## 🏗️ Architecture & Data Flow

The game execution is orchestrated by a `game_engine` executable. Our bot operates as an independent binary communicating over `STDIN` and `STDOUT`. 

### The Interaction Protocol
- **Init:** The engine starts the game assigning the bot a token (e.g., `p1` or `p2`). Our bot parses this token to understand whether we are representing `@` / `a` or `$` / `s`.
- **Game Loop:** Each turn, the engine sends the Anfield grid and the new random piece. Our bot must calculate coordinates, output `X Y\n` within standard bounds, and the turn concludes. 

### Overall Game Flow Engine Diagram

```mermaid
sequenceDiagram
    participant Docker as Docker Environment
    participant Engine as Game Engine (VM)
    participant Player1 as Our Rust Bot (P1)
    participant Player2 as Opponent Bot (P2)

    Docker->>Engine: Run ./game_engine
    Engine->>Player1: Send "exec p1" ($$$ exec p1 : [path])
    Engine->>Player2: Send "exec p2" ($$$ exec p2 : [path])
    
    loop Until Anfield is Full or Timeout
        Engine->>Player1: Write Anfield Grid (Size & Cells)
        Engine->>Player1: Write Random Piece (Size & Cells)
        Player1->>Player1: Parse input into Structs
        Player1->>Player1: Execute Algorithm & Find Best Coordinate
        Player1-->>Engine: Write "X Y\n" to STDOUT
        Engine->>Engine: Validate placement & Update Grid
        
        Engine->>Player2: Write Anfield Grid & New Piece
        Player2->>Player2: Parse & Calculate
        Player2-->>Engine: Write "X Y\n" to STDOUT
        Engine->>Engine: Validate placement & Update Grid
    end
    
    Engine->>Docker: Display Final Score & Announce Winner
```

---
## 🧠 Algorithm & Game Logic

Our bot's internal logic is cleanly layered into three steps: **Input Parsing**, **Placement Validation**, and **Strategic Selection**.

### Game Logic Sequence

```mermaid
graph TD
    A([Start Turn]) --> B[Parse STD IN]
    B --> C{Determine Payload}
    
    C -->|Grid Layout| D[Update Local Anfield State]
    C -->|Piece Layout| E[Create Local Piece Struct]
    
    D & E --> F[Brute-force Candidate Coordinates: -size to Box Size]
    F --> G{Check Validity Rules}
    
    G -->|Condition 1| H[Exactly 1 owned cell overlapping]
    G -->|Condition 2| I[Exactly 0 opponent cells overlapping]
    G -->|Condition 3| J[Strictly inside grid bounds]
    
    H & I & J --> K[Append to Valid Placements Array]
    K --> L{Any Valid Options?}
    
    L -- No --> M[Output: 0 0 - Forfeit/Wait]
    L -- Yes --> N[Calculate Opponent's Center of Mass]
    N --> O[Loop: Calc Euclidean Distance to Enemy Center for Each Option]
    O --> P[Select Option with MINIMUM Distance]
    
    P --> Q[Print Chosen X Y to STDOUT]
    M --> Q
    
    Q --> R([Wait For Next Turn])
```

---
## 💻 Technical Documentation

### Directory Structure
```text
solution/src/
├── main.rs      # Application entry-point and STDIN loop
├── parser.rs    # Converts strings from STDIN into memory structs
├── placement.rs # Mathematical overlay enforcement
├── strategy.rs  # Greedy Algorithm, Center of Mass calculation
└── types.rs     # Data structures definition (Player, Grid)
```

---
## 🚀 Getting Started & Usage Guide

We provide four distinct ways to interact with, compile, and run the Filler bot depending on your environment preference.

### 1. The Automation Script (Recommended)
This project includes a convenient automation bash script `run.sh` that wraps all the Docker and Cargo routines. It parses scores gracefully and makes verifying audits extremely simple. **(Run this outside the Docker container!)**

- **Launch Background Container**: `./run.sh docker`
- **Run the Complete Audit Suite**: `./run.sh audit` *(Runs 5 alternating matches against all standard and bonus test bots in Docker)*
  *(Note: If you have an Apple Silicon Mac, you can run `./run.sh audit m1` to bypass Docker and run the native `m1_game_engine` and `m1_robots` directly on your host system).*
- **Run Unit Tests**: `./run.sh test`
- **Compile the Custom Bot**: `./run.sh build`
- **Open Interactive Shell**: `./run.sh shell`

---

### 2. Docker Compose (Background Service)
If you prefer standard Docker Compose commands, we provide a `docker-compose.yml` natively configured for multi-arch support (including Apple M1/M2 Rosetta emulation) to prevent broken pipes.

- **Start the environment**: `docker compose up -d`
- **Stop the environment**: `docker compose down`
- **Enter the container shell**: `docker exec -it filler-bot bash`

---

### 3. Cargo Commands (Inside the Container)
Once inside the Docker container shell, you can use standard Cargo commands to compile or test the Rust binary bindings manually.

```bash
cd solution
# Build the bot in release mode natively
cargo build --release

# Run the integration and unit tests
cargo test

# Check for compiler and style warnings
cargo check
```

---

### 4. Normal Terminal Running (The Game Engine)
To run a manual game match using the official engine natively inside the shell, execute the engine with your compiled bot and a test opponent.

```bash
./linux_game_engine -f maps/map01 -p1 solution/target/release/filler -p2 linux_robots/bender
```

#### Useful Engine Flags:
| Flag | Description |
| ---- | ----------- |
| `-f` | Path to the specific map (e.g., `maps/map01`). |
| `-p1` | Path to Player 1's executable. |
| `-p2` | Path to Player 2's executable. |
| `-q` | Quiet mode (reduces console spam so you only see the final scores). |
| `-s` | Specifies a seed int for deterministic maps and pieces. |

---

## 🐳 Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## 📝 Notes

- `Terminator` is a very strong robot.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

---
## 👥 Authors

- **Sayed Ahmed Husain**