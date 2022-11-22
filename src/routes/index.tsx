import { invoke } from '@tauri-apps/api/tauri'
import { onMount } from 'solid-js';
import { A } from "solid-start";
import Counter from "~/components/Counter";

export default function Home() {
  function onClick () {
    invoke('test', { name: 'World' })
    .then(console.log)
    .catch(console.error)
  }
  return (
    <main class="text-center mx-auto text-gray-700 p-4">
      <h1 onClick={onClick} class="max-6-xs text-6xl text-sky-700 font-thin uppercase my-16">
        Hello worlda!
      </h1>
      <Counter />
      <p class="mt-8">
        Visit{" "}
        <a
          href="https://solidjs.com"
          target="_blank"
          class="text-sky-600 hover:underline"
        >
          solidjs.com
        </a>{" "}
        to learn how to build Solid apps.
      </p>
      <p class="my-4">
        <span>Home</span>
        {" - "}
        <A href="/about" class="text-sky-600 hover:underline">
          About Page
        </A>{" "}
      </p>
    </main>
  );
}
