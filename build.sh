#!/bin/bash

# Array of targets
targets=(
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  # "x86_64-unknown-linux-gnu"
  # "aarch64-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
  # "i686-pc-windows-gnu"
)

# Loop through each target and build
for target in "${targets[@]}"; do
  echo "Building for target: $target"
  cargo build --target "$target" --release
  
  # Determine the output binary name
  old_binary="node-winit-wry"
  new_binary=$target
  
  # Add .exe suffix for Windows targets
  if [[ "$target" == *"windows-gnu" ]]; then
    old_binary="$old_binary.exe"
    new_binary="$new_binary.exe"
  fi

  # Move the produced binary file
  mv target/$target/release/$old_binary binary/$new_binary
done