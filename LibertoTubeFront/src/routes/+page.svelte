<script lang="ts">
  import StatusItem from './StatusItem.svelte';
  import FancyInput from './FancyInput.svelte';
  import { onMount } from 'svelte';

  let proxy_status_data = {
    "gfbrowser.com": "availible",
    "down_example.com": "down"
  };

  let contentSection: HTMLElement | null = null;
  let lastScrollY = 0;
  let isScrolling = false;

  function scrollToContent() {
    if (contentSection) {
      contentSection.scrollIntoView({ behavior: 'smooth' });
    }
  }

  function scrollToLanding() {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }

  onMount(() => {
    const handleScroll = () => {
      if (isScrolling) return;

      if (window.scrollY > lastScrollY && window.scrollY > 10) {
        isScrolling = true;
        scrollToContent();
      } else if (window.scrollY < lastScrollY && window.scrollY < contentSection.offsetTop) {
        isScrolling = true;
        scrollToLanding();
      }

      setTimeout(() => {
        isScrolling = false;
      }, 1000); 

      lastScrollY = window.scrollY;
    };

    window.addEventListener('scroll', handleScroll);

    return () => {
      window.removeEventListener('scroll', handleScroll);
    };
  });
</script>

<style>
  .landing-section, .main-content-section {
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 2rem;
    text-align: center;
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
</style>

<!-- Landing Section -->
<div class="landing-section">
  <h1>Welcome</h1>
</div>

<!-- Main Content Section -->
<div class="main-content-section" bind:this={contentSection}>
  <div class="container">
    <h2>List of available alternative domains:</h2>
    <FancyInput {proxy_status_data} />
    <div class="col">
      {#each Object.entries(proxy_status_data) as [resource, status]}
        <StatusItem {resource} {status} />
      {/each}
    </div>
  </div>
</div>
