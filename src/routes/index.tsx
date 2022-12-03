import { invoke } from '@tauri-apps/api/tauri'
import { onMount } from 'solid-js';
import { A } from "solid-start";
import Counter from "~/components/Counter";

export default function Home() {
  function onClick () {
    invoke('get_data')
    .then((response) => console.log(response))
    .catch(console.error)
  }
  return (
    <main class="text-center mx-auto text-gray-700 p-4">
      <div onClick={onClick}>
        test
      </div>
    </main>
  );
}
