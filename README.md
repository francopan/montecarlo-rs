# Monte Carlo Estimation (Number of Sprints to Finish a Project)

This Rust program uses historical sprint data to estimate how many sprints will be needed to complete a project using Monte Carlo simulation.

Based on the [Node.JS Version](https://git.francopan.com.br/franco/montecarlo-estimation) ([Github](https://github.com/francopan/montecarlo-estimation)).


## 🚀 Running the Program

First, build and run the project:

```bash
cargo run
```


## 🧾 Prompted Inputs

When you run the program, you’ll be asked to enter:

- **Target number of story points**: Total number of story points (or stories) the project contains.
- **Percentage of sprint allocation**: Fraction of each sprint dedicated to this project, from `0.0` to `1.0`.  
  _Example: `0.75`_
- **Tasks created per 10 completed**: For every 10 tasks completed, how many new tasks are added to the backlog (usually new bugs).  
  _Example: `2`_


## ⚙️ Configuration Constants

These parameters can be modified directly in the code:

- `MAX_ROUNDS`: Maximum number of Monte Carlo simulation iterations  
  _Default: `10_000`_


## 📊 Sprint Data

The program expects a `sprints.csv` file in the root directory.  
Each row should represent a sprint and the total number of story points completed.

**Example `sprints.csv`:**
```
Sprint 1, 13
Sprint 2, 8
Sprint 3, 21
...
```


## 📈 Example Output

```text
✔ What is the target number of story points for the project?  · 34
✔ What is the percentage of story points allocated for this project over the entire sprint (from 0.0 to 1.0)?  · 0.75
✔ For every 10 tasks, how many new tasks are created/added to the sprint?  · 1

Sprint   | Total   | Confidence of completion (%)
-- | - | -
2        | 396     | 3.96
3        | 4709    | 47.09
4        | 9152    | 91.52
5        | 9968    | 99.68
6        | 9999    | 99.99
7        | 10000   | 100.00
```


## 📌 Notes

- This tool works both for story points and number of stories.
- You can tweak the logic for scope creep, velocity distribution, or allocation formula inside the code.

## 🧱 Build

This guide explains how to build a Rust project for Windows, macOS, and Linux.

### Prerequisites

1. **Install Rust**: If you don't have Rust installed, use [rustup](https://rustup.rs/) to install Rust and Cargo, the Rust package manager.

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Ensure the latest Rust version**:

   ```bash
   rustup update
   ```

### Build on Linux, macOS, and Windows

Rust makes it easy to cross-compile for multiple platforms. Here's how to set it up.

#### 1. **Targeting Windows from Linux/macOS**

To target Windows, you'll need the `x86_64-pc-windows-gnu` target and MinGW (Minimal GNU for Windows) toolchain.

##### Add the Windows target:

```bash
rustup target add x86_64-pc-windows-gnu
```

##### Install MinGW:

- **For Linux (Ubuntu/Debian-based)**:

  ```bash
  sudo apt-get install gcc-mingw-w64-x86-64
  ```

  For other Linux distributions, use your package manager to install MinGW.

- **For macOS** (using Homebrew):

  ```bash
  brew install mingw-w64
  ```

#### 2. **Build the Project for Windows**

Now you can build your Rust project for Windows. Run the following command:

```bash
cargo build --target x86_64-pc-windows-gnu --release
```

This will compile the project and generate the Windows executable in the `target/x86_64-pc-windows-gnu/release/` directory.

#### 3. **Test the Windows Binary**

To test the Windows binary on a non-Windows machine, you have a few options:

- Use a **Windows virtual machine** or **Wine** (on Linux/macOS).
- Copy the binary to a Windows machine and run it there.


### Building for macOS

If you're on Linux or Windows and want to build for macOS, follow these steps:

#### 1. **Install the macOS Target**

Add the macOS target with the following command:

```bash
rustup target add x86_64-apple-darwin
```

#### 2. **Build the Project for macOS**

Once the target is installed, you can build for macOS:

```bash
cargo build --target x86_64-apple-darwin --release
```

This will generate the macOS executable in the `target/x86_64-apple-darwin/release/` directory.


### Building for Linux

If you're on Windows or macOS and want to build for Linux, follow these steps:

#### 1. **Install the Linux Target**

Add the Linux target with the following command:

```bash
rustup target add x86_64-unknown-linux-gnu
```

#### 2. **Build the Project for Linux**

Once the target is installed, you can build for Linux:

```bash
cargo build --target x86_64-unknown-linux-gnu --release
```

This will generate the Linux executable in the `target/x86_64-unknown-linux-gnu/release/` directory.

## 🛠 Built With

- [Rust](https://www.rust-lang.org/)
- `rand` Utilities to generate random numbers
- `csv` Fast and flexible CSV reader and writer
- `serde` Serializing and deserializing Rust data structures
- `smallvec` Small vectors in various sizes