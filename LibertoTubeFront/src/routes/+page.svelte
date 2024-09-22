<script lang="ts">
  import StatusItem from './StatusItem.svelte';
  import FancyInput from './FancyInput.svelte';
  import AnimatedText from './AnimatedText.svelte';
  import { onMount } from 'svelte';
  
  let texts_to_show = ['Confident', 'Anonymous', 'Practical'];
  let proxy_status_data = {
    "gfbrowser.com/youtube": "available",
    "yetanotherexample.com": "available",
    "down_example.com": "down"
  };
  let contentSection: HTMLElement | null = null;
  let isScrolling = false;
  let windowWidth = 0;
  let showAlternativeDomains = true;
  
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
  
    const handleResize = () => {
      windowWidth = window.innerWidth;
      showAlternativeDomains = windowWidth > 768;
    };
  
    window.addEventListener('scroll', handleScroll);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('resize', handleResize);
    handleResize();
  
    return () => {
      window.removeEventListener('scroll', handleScroll);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('resize', handleResize);
    };
  });
  </script>
  
  <style>
    :global(body) {
      font-family: 'Poppins', sans-serif;
      transition: background-color 0.5s ease, color 0.5s ease;
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
  
    .landing-section, .main-content-section {
      min-height: 100vh;
      display: flex;
      justify-content: center;
      align-items: center;
      text-align: center;
      position: relative;
      padding: 2rem;
    }
  
    .landing-section {
      background: linear-gradient(135deg, #f5f5f5, #d4d4d4);
    }
  
    .main-content-section {
      background: linear-gradient(45deg, #ffffff, #d4d4d4);
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
      border-radius: 10px;
    }
  
    .container {
      width: 100%;
      max-width: 1200px;
      margin: 0 auto;
      padding: 0 1rem;
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
      0%, 20%, 50%, 80%, 100% { transform: translateY(0); }
      40% { transform: translateY(-10px); }
      60% { transform: translateY(-5px); }
    }
  
    h1, h2 {
      border-bottom: 2px solid #eaeaea;
      padding-bottom: 0.5rem;
    }
  
    .landing-title {
      font-size: clamp(2rem, 8vw, 6rem);
      margin-bottom: 1rem;
    }
  
    .subtitle {
      font-size: clamp(1rem, 4vw, 2.75rem);
      margin-bottom: 2rem;
    }
  
    .content-title {
      font-size: clamp(1.5rem, 6vw, 4rem);
    }
  
    .logo {
      height: clamp(40px, 10vw, 80px);
      margin-right: 1rem;
    }
  
    @media (max-width: 1060px) {
      .landing-section, .main-content-section {
        padding: 1rem;
      }
  
      button {
        padding: 0.75rem 1.5rem;
        font-size: 1.2rem;
      }
  
      .subtitle {
        display: none;
      }
    }
  
    .alternative-domains {
      margin-top: 2rem;
    }
  
    @media (max-width: 1280px) {
      .alternative-domains {
        display: none;
      }
    }
  </style>
  
  <div class="landing-section">
    <div class="container">
      <div class="row justify-content-center align-items-center">
        <div class="col-12">
          <div class="d-flex align-items-center justify-content-center mb-3">
            <img src="/logo.svg" alt="Logo" class="logo"/>
            <h1 class="landing-title"><b>LIBERTOTUBE</b></h1>
          </div>
          <div class="mb-3">
            <AnimatedText texts={texts_to_show} />
          </div>
          {#if windowWidth > 768}
            <p class="subtitle mb-4">
              We provide easy monitoring of alternative social media frontends and tools for easy distribution in countries with highly-restricted internet access
            </p>
          {/if}
          <button on:click={scrollToContent} type="button" class="btn btn-light border border-radius-10px">
            Try it out!
          </button>
        </div>
      </div>
      <div class="scroll-indicator">
        <i class="fas fa-chevron-down"></i>
      </div>
    </div>
  </div>
  
  <div class="main-content-section" bind:this={contentSection}>
    <div class="container">
      <h1 class="content-title mb-4"><b>Enter your YouTube URL:</b></h1>
      <div class="row justify-content-center">
        <div class="col-12 col-md-8">
          <FancyInput {proxy_status_data} />
        </div>
      </div>
      {#if showAlternativeDomains}
        <div class="alternative-domains">
          <h2 class="content-title mt-5 mb-4">List of available alternative domains:</h2>
          <div class="row justify-content-center">
            <div class="col-12 col-md-8">
              {#each Object.entries(proxy_status_data) as [resource, status]}
                <StatusItem {resource} {status} />
              {/each}
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>
  
  {#if windowWidth > 768}
    <div class="cursor-circle" style="left: {cursorCircle.x - 10}px; top: {cursorCircle.y - 10}px;"></div>
  {/if}