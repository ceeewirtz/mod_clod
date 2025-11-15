# mod_clod

Collection of experimental projects and demos.

## Projects

### 🎮 Ray-Cast Hello World

A WebAssembly-powered ray-casting engine that displays "HELLO WORLD" in a 3D maze environment.

**[View Live Demo →](https://raycast-hello.vercel.app)** *(Coming soon after first deploy)*

**Quick Deploy:**

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/ceeewirtz/mod_clod&project-name=raycast-hello&root-directory=raycast-hello&build-command=curl%20-sSf%20https://sh.rustup.rs%20%7C%20sh%20-s%20--%20-y%20%26%26%20.%20%24HOME/.cargo/env%20%26%26%20cargo%20install%20wasm-pack%20%26%26%20cd%20wasm-raycast%20%26%26%20wasm-pack%20build%20--target%20web%20%26%26%20cd%20..%20%26%26%20npm%20run%20build)

Or manually:
1. Go to [vercel.com/new](https://vercel.com/new)
2. Import this repository
3. Set Root Directory: `raycast-hello`
4. Deploy!

**Tech Stack:**
- Rust + WebAssembly (ray-casting engine)
- Next.js + React + TypeScript
- Tailwind CSS

[Read more →](./raycast-hello/README.md)

---

## Development

Each project has its own README with setup instructions.

## License

MIT
