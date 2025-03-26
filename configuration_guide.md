# Configuration Guide

This guide explains how to configure gitoxide and covers available configuration options, their meanings, and how they affect git operations.

## Table of Contents

1. [Introduction](#introduction)
2. [Configuration Sources](#configuration-sources)
3. [Core Configuration Options](#core-configuration-options)
4. [Remote Configuration](#remote-configuration)
5. [Diff and Merge Configuration](#diff-and-merge-configuration)
6. [Performance Configuration](#performance-configuration)
7. [Advanced Configuration](#advanced-configuration)

## Introduction

Gitoxide uses a configuration system similar to Git, allowing you to customize various aspects of its behavior. This guide will help you understand how to configure gitoxide effectively.

## Configuration Sources

Gitoxide reads configuration from multiple sources, in the following order of precedence (highest to lowest):

1. Command-line options
2. Environment variables
3. Repository-specific configuration (.git/config)
4. User-specific configuration (~/.gitconfig)
5. System-wide configuration (/etc/gitconfig)

## Core Configuration Options

### core.bare

- Type: Boolean
- Default: false

Determines if the repository is bare (without a working tree).

```toml
[core]
    bare = true
```

### core.logAllRefUpdates

- Type: Boolean or "always"
- Default: true for non-bare repositories, false for bare repositories

Controls whether to log all reference updates in the reflog.

```toml
[core]
    logAllRefUpdates = true
```

### core.abbrev

- Type: Integer (4-40)
- Default: 7

Sets the length of abbreviated object names (commit hashes).

```toml
[core]
    abbrev = 8
```

### core.useReplaceRefs

- Type: Boolean
- Default: true

Determines whether to use replace refs when resolving objects.

```toml
[core]
    useReplaceRefs = false
```

## Remote Configuration

### remote.<name>.url

- Type: String
- Default: None

Sets the URL for a remote repository.

```toml
[remote "origin"]
    url = "https://github.com/example/repo.git"
```

### remote.<name>.pushurl

- Type: String
- Default: None

Sets a separate URL for pushing to a remote repository.

```toml
[remote "origin"]
    pushurl = "git@github.com:example/repo.git"
```

## Diff and Merge Configuration

### diff.algorithm

- Type: String ("myers", "minimal", "patience", "histogram")
- Default: "myers"

Specifies the diff algorithm to use.

```toml
[diff]
    algorithm = "patience"
```

### merge.conflictStyle

- Type: String ("merge", "diff3")
- Default: "merge"

Sets the style of conflict markers in merge conflicts.

```toml
[merge]
    conflictStyle = "diff3"
```

## Performance Configuration

### core.useMultiPackIndex

- Type: Boolean
- Default: true

Enables or disables the use of multi-pack indices for improved performance.

```toml
[core]
    useMultiPackIndex = true
```

### core.packCacheBytes

- Type: Integer
- Default: 0 (disabled)

Sets the size of the memory-backed delta pack cache in bytes.

```toml
[core]
    packCacheBytes = 1048576  # 1 MB
```

### core.objectCacheBytes

- Type: Integer
- Default: 0 (disabled)

Sets the size of the whole object cache in bytes.

```toml
[core]
    objectCacheBytes = 52428800  # 50 MB
```

## Advanced Configuration

### gitoxide.core.refsNamespace

- Type: String
- Default: None

Sets a custom namespace for references.

```toml
[gitoxide "core"]
    refsNamespace = "custom/"
```

### core.fsmonitor

- Type: Boolean
- Default: false

Enables or disables the file system monitor for faster status checks.

```toml
[core]
    fsmonitor = true
```

### core.ignoreCase

- Type: Boolean
- Default: false on case-sensitive file systems, true on case-insensitive file systems

Determines whether gitoxide should ignore case when comparing file names.

```toml
[core]
    ignoreCase = true
```

Remember that these configuration options can be set at different levels (system, user, or repository) and can be overridden by command-line options or environment variables. Always refer to the most up-to-date documentation for the specific version of gitoxide you are using, as configuration options may change or be added over time.