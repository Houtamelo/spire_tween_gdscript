#samply record ./my-application my-arguments

# Godot editor executable path
GODOT_PATH="$HOME/bin/Godot_v4.4.1-stable_linux.x86_64"

# Project relative path
PROJECT_PATH="./spire_tween_gdscript"

# Scene to run
SCENE_PATH="benchmarks/moving_2d.tscn"

# Test setup
export TEST_NODE_AMOUNT=1000
export TEST_DURATION=10.0
export TEST_IS_BUILTIN=0

# Build Rust dynamic library
cargo build --profile samply

# Run samply
samply record "$GODOT_PATH" --path "$PROJECT_PATH" "$SCENE_PATH"