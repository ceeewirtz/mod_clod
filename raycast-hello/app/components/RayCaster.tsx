'use client';

import { useEffect, useRef, useState } from 'react';

export default function RayCaster() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [loaded, setLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let animationId: number;
    let rayCaster: any;
    const keys: Record<string, boolean> = {};

    const init = async () => {
      try {
        // Load the WASM module
        const wasm = await import('../../wasm-raycast/pkg/wasm_raycast');
        rayCaster = new wasm.RayCaster();

        const canvas = canvasRef.current;
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        const width = rayCaster.get_width();
        const height = rayCaster.get_height();
        canvas.width = width;
        canvas.height = height;

        setLoaded(true);

        // Animation loop
        const animate = () => {
          // Handle keyboard input
          const rotSpeed = 0.05;
          const moveSpeed = 0.1;

          if (keys['ArrowLeft']) {
            rayCaster.rotate(-rotSpeed);
          }
          if (keys['ArrowRight']) {
            rayCaster.rotate(rotSpeed);
          }
          if (keys['ArrowUp']) {
            rayCaster.move_forward(moveSpeed);
          }
          if (keys['ArrowDown']) {
            rayCaster.move_forward(-moveSpeed);
          }

          // Render
          const imageData = rayCaster.render();
          const imageArray = new Uint8ClampedArray(imageData);
          const imgData = new ImageData(imageArray, width, height);
          ctx.putImageData(imgData, 0, 0);

          // Draw "Hello World" text overlay
          ctx.save();
          ctx.font = 'bold 48px monospace';
          ctx.textAlign = 'center';
          ctx.textBaseline = 'top';

          // Shadow for better visibility
          ctx.shadowColor = 'black';
          ctx.shadowBlur = 10;
          ctx.shadowOffsetX = 2;
          ctx.shadowOffsetY = 2;

          // Gradient text
          const gradient = ctx.createLinearGradient(0, 50, 0, 100);
          gradient.addColorStop(0, '#FFD700');
          gradient.addColorStop(1, '#FFA500');
          ctx.fillStyle = gradient;

          ctx.fillText('HELLO WORLD', width / 2, 50);
          ctx.restore();

          // Instructions
          ctx.save();
          ctx.font = '14px monospace';
          ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
          ctx.textAlign = 'center';
          ctx.fillText('Use Arrow Keys to Move', width / 2, height - 30);
          ctx.restore();

          animationId = requestAnimationFrame(animate);
        };

        // Keyboard event listeners
        const handleKeyDown = (e: KeyboardEvent) => {
          if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
            e.preventDefault();
            keys[e.key] = true;
          }
        };

        const handleKeyUp = (e: KeyboardEvent) => {
          if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
            e.preventDefault();
            keys[e.key] = false;
          }
        };

        window.addEventListener('keydown', handleKeyDown);
        window.addEventListener('keyup', handleKeyUp);

        animate();

        return () => {
          cancelAnimationFrame(animationId);
          window.removeEventListener('keydown', handleKeyDown);
          window.removeEventListener('keyup', handleKeyUp);
        };
      } catch (err) {
        console.error('Failed to load WASM module:', err);
        setError(err instanceof Error ? err.message : 'Unknown error');
      }
    };

    init();

    return () => {
      if (animationId) {
        cancelAnimationFrame(animationId);
      }
    };
  }, []);

  return (
    <div className="flex flex-col items-center gap-4">
      <h1 className="text-3xl font-bold text-zinc-900 dark:text-zinc-100">
        WebAssembly Ray-Caster
      </h1>
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          Error: {error}
        </div>
      )}
      {!loaded && !error && (
        <div className="text-zinc-600 dark:text-zinc-400">
          Loading WASM module...
        </div>
      )}
      <canvas
        ref={canvasRef}
        className="border-4 border-zinc-800 rounded-lg shadow-2xl"
        style={{ imageRendering: 'pixelated' }}
      />
      <div className="text-sm text-zinc-600 dark:text-zinc-400 text-center max-w-md">
        <p>A classic ray-casting engine powered by WebAssembly (Rust).</p>
        <p className="mt-2">The cyan-colored walls spell out &quot;HELLO WORLD&quot; in the 3D maze!</p>
      </div>
    </div>
  );
}
