<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { get } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';

  type ImagePathObj = {
    preview: string;
    raw: string;
  };

  type ImageItem = {
    path: string | ImagePathObj;
    preview?: string;
    loading: boolean;
    error?: boolean;
  };

  type AnimationDirection = 'left' | 'right' | 'up' | 'none';

  let images: ImageItem[] = [];
  let error: string | null = null;
  let currentIndex = 0;

  let animDirection: AnimationDirection = 'none';
  let isAnimating = false;

  $: folderPath = get(page).url.searchParams.get('path');

  onMount(async () => {
    if (!folderPath) {
      error = 'Error: No path provided';
      return;
    }

    try {
      const paths = await invoke<string[]>('map_images', { path: folderPath });
      images = paths.map((p) => ({ path: p, loading: false }));

      if (images.length > 0) {
        await loadImage(currentIndex);
      }
    } catch (e) {
      console.error(e);
      error = 'Error while loading imagelist';
    }
  });

  async function loadImage(i: number) {
    const img = images[i];
    if (!img || img.loading || img.preview) return;

    img.loading = true;
    images = [...images];

    const pathString = typeof img.path === 'string' ? img.path : img.path.preview;

    try {
      const base64 = await invoke<string>('image_to_base64_data_url', { arg: pathString });
      img.preview = base64;
      images = [...images];
    } catch (e) {
      console.error(e);
      img.error = true;
    } finally {
      img.loading = false;
      images = [...images];
    }
  }

  async function nextImage() {
    if (currentIndex + 1 >= images.length || isAnimating) return;

    animDirection = 'left';
    isAnimating = true;

    setTimeout(async () => {
      currentIndex += 1;
      await loadImage(currentIndex);
      animDirection = 'none';
      isAnimating = false;
    }, 300);
  }

  async function prevImage() {
    if (currentIndex === 0 || isAnimating) return;

    animDirection = 'right';
    isAnimating = true;

    setTimeout(async () => {
      currentIndex -= 1;
      await loadImage(currentIndex);
      animDirection = 'none';
      isAnimating = false;
    }, 300);
  }

  async function editImage() {
    if (isAnimating) return;

    const current = images[currentIndex];
    if (!current) return;

    const rawPath =
    typeof current.path === 'string'
      ? current.path
      : current.path.raw;

    try {
      await invoke('copy_image_to_wanna_edit', { imagePath: rawPath });
    } catch (e) {
      console.error('Error calling copy_image_to_wanna_edit:', e);
    return;
  }

    animDirection = 'up';
    isAnimating = true;

    setTimeout(async () => {
      if (currentIndex + 1 < images.length) {
        currentIndex += 1;
        await loadImage(currentIndex);
      }
      animDirection = 'none';
      isAnimating = false;
    }, 300);
  }
</script>

<style>
  :global(#svelte) {
    height: 100%;
  }

  .viewer {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
  }


  .frame {
    position: relative;
    flex: 1;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }


  img {
    max-width: 95%;
    max-height: 95%;
    object-fit: contain;
    user-select: none;
    transition: opacity 0.2s ease;
  }

  .slide {
    position: absolute;
    transition:
    transform 0.3s ease,
    opacity 0.25s ease;
  }

  .out-left {
    transform: translateX(-120%);
    opacity: 0;
  }

  .out-right {
    transform: translateX(120%);
    opacity: 0;
  }

  .out-up {
    transform: translateY(-120%);
    opacity: 0;
  }

  .hidden {
    opacity: 0;
  }

  .spinner {
    position: absolute;
    width: 48px;
    height: 48px;
    border: 5px solid rgba(255, 255, 255, 0.15);
    border-top: 5px solid #0a84ff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .controls {
    margin-top: 1.4rem;
    margin-bottom: 2rem; 
    display: flex;
    gap: 18px;
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

  button:hover:not(:disabled) {
    background-color: #48484a;
  }

  button:active:not(:disabled) {
    transform: scale(0.97);
  }

  button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .error {
    margin-bottom: 0.8rem;
    font-size: 0.9rem;
    color: #ff453a;
  }
</style>

<div class="viewer">
  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="frame">
    {#if images[currentIndex]}
      <img
        src={images[currentIndex].preview}
        alt=""
        class="
          slide
          {animDirection === 'left' ? 'out-left' : ''}
          {animDirection === 'right' ? 'out-right' : ''}
          {animDirection === 'up' ? 'out-up' : ''}
          {images[currentIndex].loading || !images[currentIndex].preview ? 'hidden' : ''}
        "
      />

      {#if images[currentIndex].loading}
        <div class="spinner"></div>
      {/if}
    {/if}
  </div>

  <div class="controls">
    <button on:click={prevImage} disabled={currentIndex === 0}>
      Previous
    </button>

    <button on:click={editImage}>
      Smash
    </button>

    <button on:click={nextImage} disabled={currentIndex + 1 >= images.length}>
      Pass
    </button>
  </div>
</div>
