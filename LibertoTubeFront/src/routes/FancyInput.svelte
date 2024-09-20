<script>
    import { onMount } from 'svelte';
    export let proxy_status_data = {};
  
    let inputValue = '';
    let selectedDomain = '';
    const domains = Object.keys(proxy_status_data).filter(domain => proxy_status_data[domain] === "availible"); // Filter for available domains
  
    if (domains.length > 0) {
      selectedDomain = domains[0];
    }
  </script>
  
  <style>
    .fancy-input {
      display: flex;
      align-items: center;
      margin: 20px;
    }
  
    input {
      flex: 1;
      padding: 10px;
      border: 1px solid #3d3d3d;
      border-radius: 4px;
      margin-right: 10px;
    }
  
    select {
      padding: 10px;
      border: 1px solid #3d3d3d;
      background: white;
      border-radius: 4px;
      margin-right: 10px;
    }
  
    .output {
      margin-bottom: 20px;
      font-size: 1.2em;
      font-weight: bold;
    }
  </style>
  
  <div class="fancy-input">
    <input type="text" bind:value={inputValue} placeholder="Enter your input..." />
    <select bind:value={selectedDomain} disabled={domains.length === 0}>
      {#each domains as domain}
        <option value={domain}>{domain}</option>
      {/each}
    </select>
  </div>
  
  <div class="output">
    {selectedDomain}/{inputValue}
  </div>
  
  