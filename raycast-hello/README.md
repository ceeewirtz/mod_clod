# Ray-Cast Hello World

A Next.js application featuring a WebAssembly-powered ray-casting engine that displays "HELLO WORLD" in a 3D maze environment.

## Features

- **WebAssembly Ray-Caster**: High-performance 3D rendering engine written in Rust and compiled to WebAssembly
- **Interactive Controls**: Navigate through the 3D maze using arrow keys
- **Classic Ray-Casting**: Inspired by classic games like Wolfenstein 3D
- **"Hello World" in 3D**: The cyan-colored walls in the maze spell out "HELLO WORLD"
- **Next.js + TypeScript**: Modern web framework with type safety
- **Tailwind CSS**: For styling the UI

## Tech Stack

- **Frontend**: Next.js 16, React, TypeScript
- **Styling**: Tailwind CSS
- **Ray-Casting Engine**: Rust + WebAssembly (wasm-bindgen)
- **Build Tools**: wasm-pack, webpack

## Project Structure

```
raycast-hello/
├── app/
│   ├── components/
│   │   └── RayCaster.tsx       # Main component that loads WASM and renders
│   ├── page.tsx                # Home page
│   └── layout.tsx              # Root layout
├── wasm-raycast/               # Rust WASM module
│   ├── src/
│   │   └── lib.rs              # Ray-casting engine implementation
│   ├── Cargo.toml
│   └── pkg/                    # Built WASM output
├── next.config.ts              # Next.js configuration for WASM
└── package.json
```

## How It Works

### Ray-Casting Engine

The ray-casting engine is written in Rust and compiled to WebAssembly for maximum performance. It:

1. Defines an 8x8 map grid with walls and markers
2. Casts rays from the camera position to determine what's visible
3. Calculates wall heights based on distance (perspective)
4. Colors walls differently based on type:
   - Red walls: Regular maze walls
   - Cyan walls: Mark the "HELLO WORLD" text area
5. Renders ceiling (dark blue) and floor (gray) for immersion

### Controls

- **Arrow Up**: Move forward
- **Arrow Down**: Move backward
- **Arrow Left**: Rotate left
- **Arrow Right**: Rotate right

## Getting Started

### Prerequisites

- Node.js (v18 or higher)
- Rust and Cargo
- wasm-pack

### Installation

1. Install dependencies:
```bash
npm install
```

2. Build the WASM module:
```bash
cd wasm-raycast
wasm-pack build --target web
cd ..
```

3. Start the development server:
```bash
npm run dev -- --webpack
```

4. Open [http://localhost:3000](http://localhost:3000) in your browser

## Development

### Rebuilding the WASM Module

If you make changes to the Rust code:

```bash
cd wasm-raycast
wasm-pack build --target web
```

The Next.js dev server will automatically reload.

### Modifying the Map

Edit the `MAP` array in `wasm-raycast/src/lib.rs`:
- `0` = Empty space
- `1` = Regular wall (red)
- `2` = Special wall (cyan) - marks "Hello World"

## Performance

The ray-casting calculations are performed in WebAssembly, providing near-native performance. The engine renders at 640x480 resolution with real-time ray-casting calculations for each frame.

## License

MIT

## Acknowledgments

- Ray-casting technique inspired by Lode Vandevenne's tutorial
- Built with modern web technologies: Next.js, React, TypeScript, and Rust/WASM
