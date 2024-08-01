# Touch Sim

A simple Rust application that mimics the behavior of the Unix `touch` command. This program creates a file if it does not exist or updates the modification timestamp if it does. It also ensures that any necessary parent directories are created.

## Features

- Creates the specified file if it does not exist.
- Updates the file's modification timestamp if it already exists.
- Automatically creates any required parent directories.

## Prerequisites

- [Rust](https://www.rust-lang.org/), including `cargo`, the Rust package manager and build system.

## Installation

1. **Clone the Repository:**

   ```bash
   git clone <repository-url>
   cd touch_sim
