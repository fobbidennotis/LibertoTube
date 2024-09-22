<script>
  export let proxy_status_data = {};
  let inputValue = '';
  let selectedDomain = '';
  let dropdownOpen = false;
  let domains = [];
 
  $: domains = Object.keys(proxy_status_data).filter(domain => proxy_status_data[domain] === "available");
  $: if (domains.length > 0 && !selectedDomain) {
    selectedDomain = domains[0];
  }
 
  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
  }
 
  function selectDomain(domain) {
    selectedDomain = domain;
    dropdownOpen = false;
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
 
  input, .custom-dropdown {
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
 
  .custom-dropdown {
    position: relative;
    cursor: pointer;
  }
 
  .dropdown-button {
    padding: 15px;
    border: 2px solid #3d3d3d;
    border-radius: 10px;
    background-color: #e8e8e8;
    font-size: clamp(1rem, 2vw, 1.5rem);
    transition: border-color 0.3s ease;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
 
  .dropdown-button:hover,
  .dropdown-button:focus {
    outline: none;
    border-color: #ffcc00;
  }
 
  .dropdown-menu {
    position: absolute;
    top: 100%;
    left: 0;
    background-color: #e8e8e8;
    width: auto;
    border: 2px solid #3d3d3d;
    border-radius: 10px;
    z-index: 1000;
    max-height: 300px;
    overflow-y: auto;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    display: block !important;
 }
 
  .dropdown-menu h2 {
    padding: 10px;
    cursor: pointer;
    font-size: clamp(0.9rem, 1.8vw, 1.3rem);
    margin: 0;
  }
 
  .dropdown-menu h2:hover {
    background-color: #ffcc00;
  }
 
  .output {
    margin-top: 20px;
    font-size: clamp(1rem, 2vw, 1.5rem);
    font-weight: bold;
    text-align: center;
    color: #3d3d3d;
  }
 
  .dropdown-arrow {
    margin-left: 10px;
    transition: transform 0.3s ease;
  }
 
  .dropdown-arrow.open {
    transform: rotate(180deg);
  }
 
  @media (max-width: 600px) {
    .fancy-input {
      padding: 0 10px;
    }
  }
 </style>
 
 <div class="fancy-input-container">
  <div class="fancy-input">
    <input type="text" bind:value={inputValue} placeholder="Enter your URL..." />
    <div class="custom-dropdown">
      <div class="dropdown-button" on:click={toggleDropdown}>
        <span>{selectedDomain || 'Select Domain'}</span>
        <span class="dropdown-arrow" class:open={dropdownOpen}>▼</span>
      </div>
      {#if dropdownOpen}
        <div class="dropdown-menu">
          {#each domains as domain}
            <h2 on:click={() => selectDomain(domain)}>{domain}</h2>
          {/each}
        </div>
      {/if}
    </div>
  </div>
  <div class="output">
    <h1><a style="color: #ff9900;" href="https://{selectedDomain}/{inputValue.split('/').pop()}"><b>{selectedDomain}/{inputValue.split('/').pop()}</b></a></h1>
  </div>
 </div>