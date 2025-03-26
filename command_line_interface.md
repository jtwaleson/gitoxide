# Command-Line Interface

This document provides a comprehensive guide on using the gitoxide command-line interface, including available commands, their options, and example usage.

## Table of Contents

1. [Introduction](#introduction)
2. [Basic Usage](#basic-usage)
3. [Commands](#commands)
   - [Plumbing Commands](#plumbing-commands)
   - [Porcelain Commands](#porcelain-commands)
4. [Options](#options)
5. [Examples](#examples)

## Introduction

Gitoxide is a Git implementation in Rust, providing both plumbing and porcelain commands. The command-line interface allows users to interact with Git repositories using these commands.

## Basic Usage

To use gitoxide, open your terminal and run the `gitoxide` command followed by the desired subcommand and options:

```
gitoxide <command> [options]
```

## Commands

Gitoxide provides two types of commands: plumbing and porcelain.

### Plumbing Commands

Plumbing commands are low-level commands that expose the internal workings of Git. These commands are located in the `src/plumbing/mod.rs` file.

Available plumbing commands:

- `main`: The main plumbing command (details not provided in the given context)
- `show_progress`: Displays progress information (implementation in `src/plumbing/progress.rs`)

### Porcelain Commands

Porcelain commands are high-level commands that provide a user-friendly interface for common Git operations. These commands are located in the `src/porcelain/mod.rs` file.

Available porcelain commands:

- `main`: The main porcelain command (details not provided in the given context)

## Options

Both plumbing and porcelain commands may have various options. The options are defined in their respective `options.rs` files:

- Plumbing options: `src/plumbing/options.rs`
- Porcelain options: `src/porcelain/options.rs`

To view the available options for a specific command, use the `--help` flag:

```
gitoxide <command> --help
```

## Examples

As the specific details of the commands and their options are not provided in the given context, we cannot provide concrete examples. However, here's a general structure for using gitoxide commands:

1. Basic command usage:
   ```
   gitoxide <command>
   ```

2. Command with options:
   ```
   gitoxide <command> --option1 value1 --option2 value2
   ```

3. Displaying help for a command:
   ```
   gitoxide <command> --help
   ```

4. Using a plumbing command (e.g., show_progress):
   ```
   gitoxide show_progress
   ```

5. Using a porcelain command (e.g., main):
   ```
   gitoxide main
   ```

For more detailed examples and usage instructions, please refer to the specific command documentation or use the `--help` flag with each command.