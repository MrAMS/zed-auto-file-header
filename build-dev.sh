#!/bin/bash
cd server
cargo build --release
cp target/release/auto-header-server ../
echo "✓ Server built and copied to project root"
echo "  Reload Zed extension to use updated binary"
