<script lang="ts">
  import { open, message } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';
  import { tick } from 'svelte';

  let selectedPath: string | null = null;
  let animateOut = false;

  async function openFileDialog() {
    const file = await open({
      multiple: false,
      directory: true
    });

    if (!file) return;

    selectedPath = file as string;

    let hasImages = false;

    try {
      hasImages = await invoke<boolean>('contains_images', {
        path: selectedPath
      });
    } catch {
      await message('Folder could not be checked.', {
        title: 'Error',
        kind: 'error'
      });
      return;
    }

    if (!hasImages) {
      await message(
        'The selected folder contains no JPG, PNG or supported RAW files.',
        {
          title: 'No images found',
          kind: 'warning'
        }
      );
      return;
    }

    animateOut = true;
    await tick();

    setTimeout(() => {
      goto(`/nextscreen?path=${encodeURIComponent(selectedPath!)}`);
    }, 300);
  }
</script>

<style>

.emoji_face {
  font-size: 3rem;
  margin: 0 0 0.5rem;
  background: linear-gradient(
    90deg,
    #ff0000, #ff7f00, #ffff00, #00ff00, #0000ff, #4b0082, #9400d3
  );
  background-size: 400% 100%;
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  animation: rainbow 3s linear infinite;
}

@keyframes rainbow {
  0% { background-position: 0% 50%; }
  100% { background-position: 100% 50%; }
}


h1 {
  margin: 0 0 0.5rem;
  font-size: 1.5rem;
  font-weight: 600;
}

p {
  margin: 0 0 2rem;
  font-size: 0.95rem;
  line-height: 1.5;
}

button {
  appearance: none;
  border: none;
  border-radius: 8px;
  padding: 0.6rem 1.4rem;

  font-size: 0.95rem;
  font-weight: 500;
  cursor: pointer;

  color: #ffffff;
  background-color: #3a3a3c;

  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.08);

  transition:
    background-color 0.15s ease,
    transform 0.12s ease;
}

button:hover {
  background-color: #48484a;
}

button:active {
  transform: scale(0.97);
}

.out {
  transform: translateX(-100%);
  opacity: 0;
  transition: transform 0.3s ease, opacity 0.3s ease;
}
</style>

<div
  class:out={animateOut}
  in:fly={{ x: 300, duration: 400 }}
  style="
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
  "
>
  <h1 class="emoji_face">≧◡≦</h1>
  <h1>Choose folder</h1>
  <p>Choose a folder with pictures to continue</p>

  <button on:click={openFileDialog}>
    Open folder
  </button>
</div>
