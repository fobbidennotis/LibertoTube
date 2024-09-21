<script lang="ts">
  import StatusItem from './StatusItem.svelte';
  import FancyInput from './FancyInput.svelte';
  import AnimatedText from './AnimatedText.svelte';
  import { onMount } from 'svelte';

  let texts_to_show = ['High-speed', 'Anonymous', 'Practical'];

  let proxy_status_data = {
    "gfbrowser.com": "available",
    "yetanotherexample.com": "available",
    "down_example.com": "down"
  };

  let contentSection: HTMLElement | null = null;
  let isScrolling = false;

  let cursorCircle = {
    x: 0,
    y: 0
  };

  function scrollToContent() {
    if (contentSection) {
      document.body.style.overflow = 'hidden'; 
      contentSection.scrollIntoView({ behavior: 'smooth' });

      setTimeout(() => {
        document.body.style.overflow = ''; 
      }, 1000);
    }
  }

  onMount(() => {
    const handleScroll = () => {
      if (window.scrollY > 0 && !isScrolling) {
        isScrolling = true;
        scrollToContent();

        setTimeout(() => {
          isScrolling = false;
        }, 1000); 
      }
    };

    const handleMouseMove = (event: MouseEvent) => {
      cursorCircle.x = event.clientX;
      cursorCircle.y = event.clientY;
    };

    window.addEventListener('scroll', handleScroll);
    window.addEventListener('mousemove', handleMouseMove);

    return () => {
      window.removeEventListener('scroll', handleScroll);
      window.removeEventListener('mousemove', handleMouseMove);
    };
  });
</script>

<style>
  body {
    font-family: 'Poppins', sans-serif;
    transition: background-color 0.5s ease, color 0.5s ease;
  }

  .landing-section, .main-content-section {
    height: 100vh;
    display: flex;
    justify-content: center;
    font-size: 2rem;
    text-align: center;
    position: relative;
  }

  .landing-section {
    background: linear-gradient(135deg, #f5f5f5, #d4d4d4);
  }

  .main-content-section {
    background: linear-gradient(45deg, #ffffff, #d4d4d4);
    padding: 2rem;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    border-radius: 10px;
  }

  .container {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
  }

  .cursor-circle {
    position: fixed;
    width: 15px;
    height: 15px;
    background-color: orange;
    border-radius: 50%;
    pointer-events: none;
    box-shadow: 0 0 10px rgba(255, 165, 0, 0.7);
    left: 0;
    top: 0;
    transition: transform 0.2s ease, width 0.2s, height 0.2s, box-shadow 0.2s;
  }

  .cursor-circle:hover {
    width: 20px;
    height: 20px;
    box-shadow: 0 0 20px rgba(255, 165, 0, 1);
  }

  button {
    padding: 1rem 2rem;
    font-size: 1.5rem;
    border-radius: 10px;
    background-color: #ffcc00;
    color: #333;
    border: none;
    transition: background-color 0.3s ease, transform 0.3s ease;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  button:hover {
    background-color: #ff9900;
    transform: scale(1.05);
  }

  .scroll-indicator {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    animation: bounce 1s infinite;
  }

  @keyframes bounce {
    0%, 20%, 50%, 80%, 100% {
      transform: translateY(0);
    }
    40% {
      transform: translateY(-10px);
    }
    60% {
      transform: translateY(-5px);
    }
  }

  h1, h2 {
    border-bottom: 2px solid #eaeaea;
    padding-bottom: 0.5rem;
  }
</style>

<!-- Landing Section -->
<div class="landing-section">
  <div class="col">    
    <div class="row">
      <h1 style="font-size: 300%;" class="landing-title m-3">LIBERTOTUBE</h1>
    </div>
    <div class="row">
      <span class="d-flex justify-content-center">
        <AnimatedText texts={texts_to_show} /> social media frontends
      </span>
    </div> 
    <div class="row mt-4">
      <span class="d-flex justify-content-center">
        <button on:click={scrollToContent} type="button" class="btn btn-light border border-radius">Try it out!</button>
      </span>
    </div>
    <div class="scroll-indicator">
      <i class="fas fa-chevron-down"></i>
    </div>
  </div>
  <div class="cursor-circle" style="left: {cursorCircle.x - 10}px; top: {cursorCircle.y - 10}px;"></div>
</div>

<!-- Main Content Section -->
<div class="main-content-section" bind:this={contentSection}>
  <div class="container">
    <h1>Enter your youtube URL:</h1>
    <FancyInput {proxy_status_data} />
    <h2>List of available alternative domains:</h2>
    <div class="col">
      {#each Object.entries(proxy_status_data) as [resource, status]}
        <StatusItem {resource} {status} />
      {/each}
    </div>
  </div>
  <div class="cursor-circle" style="left: {cursorCircle.x - 10}px; top: {cursorCircle.y - 10}px;"></div>
</div>
