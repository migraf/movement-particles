#!/bin/bash

# Build script for movement-particles

set -e

echo "🦀 Building WASM module..."
cd crates/wasm-bridge
wasm-pack build --target web --out-dir ../../web/wasm-pkg
cd ../..

echo "📦 Installing web dependencies..."
cd web
npm install

echo "✨ Build complete!"
echo ""
echo "🚀 Starting development server..."
echo "   Open http://localhost:3000 in your browser"
echo ""
npm run dev

