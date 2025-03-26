# Troubleshooting

This guide aims to help you resolve common issues you might encounter while using gitoxide. If you're experiencing problems, please review the following sections for potential solutions and troubleshooting steps.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Build Failures](#build-failures)
3. [Runtime Errors](#runtime-errors)
4. [Performance Issues](#performance-issues)
5. [Compatibility Problems](#compatibility-problems)

## Installation Issues

### Cargo Install Fails

If you're having trouble installing gitoxide using cargo, try the following:

1. Ensure you have the latest version of Rust installed:
   ```
   rustup update
   ```

2. If SSL certificate issues occur during clones, try omitting the `--locked` flag:
   ```
   cargo install gitoxide --no-default-features --features max-pure
   ```

3. For a more compatible installation, use:
   ```
   cargo install gitoxide --no-default-features --features max-pure
   ```

### Missing Dependencies

Some platforms may require additional tools. For example, on Fedora:

- Install `perl` for OpenSSL to build properly:
  ```
  dnf install perl
  ```

## Build Failures

If you encounter build failures when compiling gitoxide from source:

1. Ensure you have the minimum supported Rust version (check the [CI configuration](https://github.com/GitoxideLabs/gitoxide/blob/main/.github/workflows/msrv.yml#L23)).

2. Try building with different feature flags:
   ```
   cargo build --no-default-features --features max-pure
   ```

3. If using the `max` feature, ensure you have `cmake` installed on your system.

4. For smaller binaries and faster build times, use the `lean` feature:
   ```
   cargo build --no-default-features --features lean
   ```

## Runtime Errors

If you encounter errors while running gitoxide commands:

1. Ensure you're using the latest version of gitoxide:
   ```
   cargo install gitoxide --force
   ```

2. Check for any error messages in the console output and refer to the specific error sections below.

3. If the issue persists, consider [opening an issue](https://github.com/GitoxideLabs/gitoxide/issues) on the GitHub repository.

## Performance Issues

If you're experiencing slow performance:

1. Ensure you're using the `max` feature for optimal performance:
   ```
   cargo install gitoxide --features max
   ```

2. For large repositories, consider using sparse checkouts to reduce the amount of data processed.

3. Verify that your system meets the minimum hardware requirements for the size of the repositories you're working with.

## Compatibility Problems

If you're having issues with gitoxide compatibility:

1. Ensure you're using a compatible version of Git (check the [README.md](https://github.com/GitoxideLabs/gitoxide/blob/main/README.md) for compatibility information).

2. If working with remote repositories, verify that the server supports the Git protocols used by gitoxide.

3. For Windows users, ensure you're using the latest Windows build from the [releases section](https://github.com/GitoxideLabs/gitoxide/releases).

If you continue to experience issues after trying these troubleshooting steps, please [open an issue](https://github.com/GitoxideLabs/gitoxide/issues) on the GitHub repository with a detailed description of your problem, including your system information and the steps to reproduce the issue.