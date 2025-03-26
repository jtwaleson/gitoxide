# Getting Started with gitoxide

Welcome to gitoxide, a pure-rust implementation of git that provides powerful functionality and excellent performance. This guide will help you get started with gitoxide, covering installation, basic usage, and an overview of key features.

## Table of Contents

1. [Installation](#installation)
2. [Basic Usage](#basic-usage)
3. [Key Features](#key-features)
4. [Next Steps](#next-steps)

## Installation

There are several ways to install gitoxide:

### Download a Binary Release

The easiest way to install gitoxide is by using `cargo binstall`:

1. Install `cargo binstall` if you haven't already:
   ```
   cargo install cargo-binstall
   ```

2. Install gitoxide:
   ```
   cargo binstall gitoxide
   ```

### From Source via Cargo

If you prefer to build from source or need the latest development version, you can use Cargo:

```sh
# Install the default 'max' version (fastest, but requires cmake)
cargo install gitoxide

# For a pure-Rust build with fewer dependencies
cargo install gitoxide --locked --no-default-features --features max-pure

# For smaller binaries and faster build times (less fancy CLI)
cargo install gitoxide --locked --no-default-features --features lean
```

### Other Installation Methods

For other installation methods, including package managers and Docker, please refer to the [full installation guide](https://github.com/GitoxideLabs/gitoxide#installation) in the README.

## Basic Usage

After installation, you'll have access to two main binaries:

1. `ein`: High-level commands for everyday use, optimized for user experience.
2. `gix`: Low-level commands for specialized cases and testing new features.

Here are some basic examples to get you started:

### Initialize a Repository

```sh
ein init my_project
cd my_project
```

### Clone a Repository

```sh
ein clone https://github.com/example/repo.git
```

### Fetch Changes

```sh
ein fetch origin
```

### Create a Commit

```sh
# Add files to the staging area
ein add .

# Create a commit
ein commit -m "Initial commit"
```

### Push Changes

```sh
ein push origin main
```

## Key Features

gitoxide offers several key features that make it stand out:

1. **Pure Rust Implementation**: Provides excellent performance and memory safety.
2. **Cross-platform Support**: Works on Linux, macOS, and Windows.
3. **Flexible Trust Model**: Allows fine-grained control over security and trust levels.
4. **High Performance**: Optimized for speed and efficiency, especially in object access.
5. **Thread-safe Operations**: Supports concurrent operations with `ThreadSafeRepository`.
6. **Extensive API**: Provides both high-level and low-level APIs for various git operations.

## Next Steps

To dive deeper into gitoxide, consider exploring the following resources:

1. [API Documentation](https://docs.rs/gitoxide): Detailed information about the gitoxide API.
2. [GitHub Repository](https://github.com/GitoxideLabs/gitoxide): Source code, issues, and contributing guidelines.
3. [Crate Status](https://github.com/GitoxideLabs/gitoxide/blob/main/crate-status.md): Current development status of various gitoxide components.

For more advanced usage and examples, refer to the individual module documentation within the gitoxide crate.

Happy coding with gitoxide!