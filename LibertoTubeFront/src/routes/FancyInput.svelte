<script lang="ts">
  import { onMount } from 'svelte';

  let inputValue = '';
  let selectedDomain = '';
  let domains: string[] = [];


  onMount(async () => {
    try {
      const res = await fetch('http://127.0.0.1:7777/'); 
      domains = await res.json(); 
      console.log(domains)
      
      selectedDomain = domains[0]; // Set default to the first domain
    } catch (err) {
      console.error('Error fetching domains:', err);
    }
  });

  // Function to cycle through domains
  function cycleDomain() {
    const currentIndex = domains.indexOf(selectedDomain);
    const nextIndex = (currentIndex + 1) % domains.length;
    selectedDomain = domains[nextIndex];
  }
</script>

<style>
  .fancy-input-container {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
  }

  .fancy-input {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
  }

  input {
    width: 100%;
    max-width: 600px;
    margin: 10px 0;
  }

  input {
    padding: 15px;
    border: 2px solid #3d3d3d;
    border-radius: 10px;
    background-color: #e8e8e8;
    font-size: clamp(1rem, 2vw, 1.5rem);
    transition: border-color 0.3s ease;
  }

  input:focus {
    outline: none;
    border-color: #ffcc00;
  }

  .output {
    margin-top: 20px;
    font-size: clamp(1rem, 2vw, 1.5rem);
    font-weight: bold;
    text-align: center;
    color: #3d3d3d;
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
    cursor: pointer;
  }

  button:hover {
    background-color: #ff9900;
    transform: scale(1.05);
  }
</style>

<div class="fancy-input-container">
  <div class="fancy-input">
    <input type="text" bind:value={inputValue} placeholder="Ссылка..." />
  </div>

  <div class="output">
    <h1>
      <a style="color: #ff9900;" href="https://{selectedDomain}/{inputValue.split('/').pop()}">
        <b>{selectedDomain}/{inputValue.split('/').pop()}</b>
      </a>
    </h1>
  </div>

  <!-- Button to cycle through domains -->
  <button on:click={cycleDomain} type="button" class="btn btn-light border border-radius-10px">
    Другой домен
  </button>
</div>
