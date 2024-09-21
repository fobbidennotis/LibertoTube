<script lang="ts">
  import StatusItem from './StatusItem.svelte';
  import FancyInput from './FancyInput.svelte';
  import AnimatedText from './AnimatedText.svelte';
  import { onMount } from 'svelte';

  let texts_to_show = ['High-speed', 'Anonymous', 'Practical'];

  let proxy_status_data = {
    "gfbrowser.com": "available",
    "down_example.com": "down"
  };

  let contentSection: HTMLElement | null = null;
  let isScrolling = false;

  // Cursor circle position
  let cursorCircle = {
    x: 0,
    y: 0
  };

  function scrollToContent() {
    console.log(contentSection)
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
  .landing-section, .main-content-section {
    height: 100vh;
    display: flex;
    justify-content: center;
    font-size: 2rem;
    text-align: center;
    position: relative; /* Position relative for absolute positioning of the circle */
  }

  .landing-section {
    background-color: #f5f5f5;
  }

  .main-content-section {
    background-color: #ffffff;
    padding: 2rem;
  }

  .container {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
  }

  .cursor-circle {
    position: fixed;
    width: 15px; /* Circle size */
    height: 15px;
    background-color: orange;
    border-radius: 50%;
    pointer-events: none; /* Prevent it from interfering with mouse events */
    left: 0; /* Reset left and top */
    top: 0; /* Reset left and top */
  }
</style>

<!-- Landing Section -->
<div class="landing-section">
  <div class="col">    
    <div class="row">
      <h1 style="font-size: 300%;" class="landing-title m-3">LIBERTOTUBE</h1>
    </div>
    <div class="row">
      <span class="d-flex flex justify-content-center">
        <AnimatedText texts={texts_to_show} /> social media frontends
      </span>
    </div> 
    <div class="row mt-4">
      <span class="d-flex flex justify-content-center">
        <button on:click={(e) => scrollToContent()} type="button" class="btn btn-light border border-radius" style="width: 25%; font-size: 150%;">Learn More</button>
      </span>
    </div>
  </div>
  <div class="cursor-circle" style="left: {cursorCircle.x - 10}px; top: {cursorCircle.y - 10}px;"></div>
</div>

<!-- Main Content Section -->
<div class="main-content-section" bind:this={contentSection}>
  <div class="container">
    <h1>Enter your URL:</h1>
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