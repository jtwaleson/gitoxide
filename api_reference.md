# API Reference

This document provides a comprehensive API reference for the gix library, documenting all public types, traits, and functions, along with usage examples.

## Table of Contents

1. [Repository](#repository)
2. [Object](#object)
3. [Reference](#reference)
4. [Commit](#commit)
5. [Tree](#tree)
6. [Blob](#blob)
7. [Tag](#tag)
8. [Id](#id)
9. [Config](#config)
10. [Remote](#remote)
11. [Index](#index)
12. [Diff](#diff)
13. [Merge](#merge)
14. [Status](#status)
15. [Worktree](#worktree)

## Repository

The `Repository` struct is the main entry point for working with Git repositories in gix.

### Opening a Repository

```rust
use gix::Repository;

// Open an existing repository
let repo = Repository::open("/path/to/repo")?;

// Open a bare repository
let bare_repo = Repository::open_bare("/path/to/bare/repo")?;

// Discover a repository from a path within it
let discovered_repo = Repository::discover("/path/within/repo")?;
```

### Repository Operations

```rust
// Get the repository's path
let path = repo.path();

// Check if the repository is bare
let is_bare = repo.is_bare();

// Get the repository's configuration
let config = repo.config()?;

// Get the repository's index
let index = repo.index()?;

// Get the HEAD reference
let head = repo.head()?;
```

## Object

The `Object` struct represents a Git object (blob, tree, commit, or tag).

### Working with Objects

```rust
// Find an object by its ID
let object = repo.find_object(object_id)?;

// Get the object's type
let object_type = object.kind();

// Get the object's ID
let id = object.id();

// Convert the object to a specific type
let commit = object.into_commit();
let tree = object.into_tree();
let blob = object.into_blob();
let tag = object.into_tag();
```

## Reference

The `Reference` struct represents a Git reference (branch, tag, or symbolic ref).

### Working with References

```rust
// Find a reference by name
let reference = repo.find_reference("refs/heads/main")?;

// Get the reference's name
let name = reference.name();

// Get the reference's target
let target = reference.target();

// Resolve a symbolic reference
let resolved = reference.resolve()?;

// Create a new reference
let new_ref = repo.reference("refs/heads/new-branch", commit_id, false, "New branch")?;

// Delete a reference
repo.reference_remove("refs/heads/old-branch")?;
```

## Commit

The `Commit` struct represents a Git commit object.

### Working with Commits

```rust
// Create a new commit
let new_commit = repo.commit("HEAD", &signature, &signature, "Commit message", &tree, &[&parent_commit])?;

// Get commit metadata
let author = commit.author();
let committer = commit.committer();
let message = commit.message();

// Get the commit's tree
let tree = commit.tree()?;

// Get the commit's parents
let parents = commit.parents();
```

## Tree

The `Tree` struct represents a Git tree object.

### Working with Trees

```rust
// Get a tree entry by name
let entry = tree.get_name("file.txt")?;

// Iterate over tree entries
for entry in tree.iter() {
    println!("{} {}", entry.name(), entry.id());
}

// Get the number of entries in the tree
let entry_count = tree.len();
```

## Blob

The `Blob` struct represents a Git blob object (file contents).

### Working with Blobs

```rust
// Get the blob's content
let content = blob.content();

// Get the blob's size
let size = blob.size();
```

## Tag

The `Tag` struct represents a Git tag object.

### Working with Tags

```rust
// Create a new tag
let new_tag = repo.tag("v1.0.0", &object, &signature, "Tag message", false)?;

// Get tag metadata
let name = tag.name();
let target = tag.target()?;
let tagger = tag.tagger();
let message = tag.message();
```

## Id

The `Id` struct represents a Git object ID (SHA-1 hash).

### Working with IDs

```rust
// Parse an ID from a string
let id = Id::from_str("1234567890abcdef1234567890abcdef12345678")?;

// Get the ID as a string
let id_str = id.to_string();
```

## Config

The `Config` struct represents the Git configuration.

### Working with Configuration

```rust
// Get a configuration value
let value = config.get_string("user.name")?;

// Set a configuration value
config.set_str("user.email", "user@example.com")?;

// Remove a configuration entry
config.remove("core.autocrlf")?;
```

## Remote

The `Remote` struct represents a remote repository.

### Working with Remotes

```rust
// List all remotes
let remotes = repo.remotes()?;

// Find a remote by name
let remote = repo.find_remote("origin")?;

// Add a new remote
repo.remote("upstream", "https://github.com/example/repo.git")?;

// Fetch from a remote
remote.fetch(&["main"], None, None)?;

// Push to a remote
remote.push(&["refs/heads/main:refs/heads/main"], None)?;
```

## Index

The `Index` struct represents the Git index (staging area).

### Working with the Index

```rust
// Add a file to the index
index.add_path("file.txt")?;

// Remove a file from the index
index.remove_path("old_file.txt")?;

// Write the index to disk
index.write()?;

// Read the index from disk
index.read(true)?;
```

## Diff

The `Diff` struct represents the differences between two trees, commits, or the working directory.

### Working with Diffs

```rust
// Create a diff between two trees
let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)?;

// Iterate over diff deltas
for delta in diff.deltas() {
    println!("{:?}", delta);
}

// Get the number of files changed, insertions, and deletions
let stats = diff.stats()?;
println!("Files changed: {}", stats.files_changed());
println!("Insertions: {}", stats.insertions());
println!("Deletions: {}", stats.deletions());
```

## Merge

The `Merge` struct provides functionality for merging branches or commits.

### Performing Merges

```rust
// Merge a branch into the current branch
let merge_result = repo.merge(&["refs/heads/feature-branch"], None, None)?;

// Check if the merge resulted in conflicts
if merge_result.conflicts().is_empty() {
    println!("Merge successful!");
} else {
    println!("Merge resulted in conflicts.");
}
```

## Status

The `Status` struct represents the status of the working directory.

### Checking Repository Status

```rust
// Get the status of the repository
let statuses = repo.statuses(None)?;

// Iterate over status entries
for entry in statuses.iter() {
    println!("{:?} {:?}", entry.status(), entry.path());
}
```

## Worktree

The `Worktree` struct represents a Git worktree.

### Working with Worktrees

```rust
// List all worktrees
let worktrees = repo.worktrees()?;

// Add a new worktree
repo.worktree("new-worktree", Path::new("/path/to/new/worktree"), None)?;

// Prune worktrees
repo.prune_worktrees()?;
```

This API reference provides an overview of the main components and operations available in the gix library. For more detailed information on specific functions and their parameters, please refer to the inline documentation in the source code.