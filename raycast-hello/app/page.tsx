import RayCaster from './components/RayCaster';

export default function Home() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-gradient-to-br from-zinc-900 to-zinc-800 p-8">
      <main className="flex flex-col items-center justify-center">
        <RayCaster />
      </main>
    </div>
  );
}
