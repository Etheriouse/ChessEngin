# :chess_pawn: ChessEngin

**ChessEngin** is a personal chess engine project, currently being rebuilt in **Rust**.

The goal is to develop a chess game with a graphical interface and an engine capable of choosing moves using classical search algorithms, without machine learning.

## :arrows_counterclockwise: Project Evolution

The project started in **C++**, was then explored in **Java**, and is now moving to **Rust**.

Each version is kept in its own branch (`c++`, `java`, and `rust`) to document the project's evolution.

## :jigsaw: Planned Architecture

The Rust version is being designed around the following components:

- **Board representation:** Using `Option<Pos>` to distinguish occupied and empty squares.
- **Attack detection:** An `is_attack` function to identify attacked squares and support move validation.
- **Move search:** **Minimax with Alpha-Beta pruning** to explore possible moves efficiently.
- **Move ordering:** A simple heuristic to prioritize promising moves and improve pruning efficiency.
- **Static evaluation:** A handcrafted evaluation function to estimate the strength of a position without machine learning.
- **Graphical interface:** Using **Raylib** to display the board and interact with the game.

## :construction: Development Status

**Work in progress.**

The search engine and its decision-making logic are planned features and are not yet fully implemented.

### Future Improvements

- Experiment with parallel search to take advantage of multiple CPU cores.
- Benchmark search performance.
- Improve move ordering and evaluation heuristics.

##  Technologies

- **Rust** — Main development language
- **Raylib** — Graphical interface

---

*Personal project focused on chess programming, algorithms, and performance optimization.*