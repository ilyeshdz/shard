# Shard

A minimalist shell orchestration language that transpiles to POSIX shell scripts.

## Quick Start

```bash
# Install
git clone https://github.com/ilyeshdz/shard.git
cd shard
cargo install --path .

# Create a script
echo "name = 'World'" > hello.shard
echo "echo 'Hello' name" >> hello.shard

# Build and run
shard build -i hello.shard -o hello.sh
sh hello.sh
```

## Documentation

Full documentation is available at: **https://ilyeshdz.github.io/shard-docs/**

## License

MIT
