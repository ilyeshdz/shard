# Welcome to Shard!

A minimalist shell orchestration language that transpiles to POSIX shell scripts.

## Quick Start

```bash
# Transpile and run
shard build -i main.shard -o script.sh
sh script.sh

# Or just check syntax
shard check -i main.shard
```

## Example

```shard
name = 'World'
echo 'Hello' name
```

## Commands

- `shard check` - Check syntax without generating output
- `shard build` - Build to a shell script
- `shard transpile` - Transpile to stdout

## Documentation

See [README.md](../README.md) for full documentation.
