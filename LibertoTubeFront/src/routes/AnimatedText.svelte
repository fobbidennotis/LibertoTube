<script lang="ts">
    import { fly } from 'svelte/transition';
    import { writable } from 'svelte/store';
    import { onMount } from 'svelte';
  
    export let texts: string[] = []; // Accept the array as a prop
  
    let currentText = writable(texts[0]);
    let currentIndex = 0;
    let isVisible = true;
  
    const switchText = () => {
      isVisible = false;
      setTimeout(() => {
        currentIndex = (currentIndex + 1) % texts.length;
        currentText.set(texts[currentIndex]);
        isVisible = true;
      }, 500); // Time for the text to move out before the next one moves in
    };
  
    onMount(() => {
      const intervalId = setInterval(switchText, 2000); // Wait 2 seconds between transitions
  
      return () => {
        clearInterval(intervalId);
      };
    });
  </script>
  
  <style>
    .text-box {
      position: relative;
      height: 6rem; /* Adjust to match the height of your text */
      overflow: hidden;
      display: flex;
      justify-content: center; /* Centering text */
      align-items: center;
      width: 100%; /* Full width */
    }
  
    .text {
      position: relative;
      text-align: center;
      font-size: 5rem; /* Default text size */
      transition: font-size 0.3s ease; /* Smooth font size transition */
    }
  
    /* Responsive adjustments */
    @media (max-width: 768px) {
      .text {
        font-size: 3rem; /* Adjust text size for smaller screens */
      }
    }
  
    @media (max-width: 480px) {
      .text {
        font-size: 2rem; /* Further adjust for mobile */
      }
    }
  </style>
  
  <div class="text-box">
    {#if isVisible}
      <div class="text" transition:fly={{ y: 50, duration: 500 }}>
        {#await $currentText}
          Loading...
        {:then text}
          {text}
        {/await}
      </div>
    {:else}
      <div class="text" transition:fly={{ y: -50, duration: 500 }}></div>
    {/if}
  </div>
  