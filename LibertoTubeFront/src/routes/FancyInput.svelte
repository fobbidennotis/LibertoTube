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
  /* Use more specific selectors to increase CSS specificity */
  .fancy-input-container .fancy-input {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 20px;
    position: relative;
  }
  .fancy-input-container input {
    margin: 5px;
    padding: 10px;
    border: 2px solid #3d3d3d;
    border-radius: 10px;
    background-color: #e8e8e8;
    width: 40%;
    font-size: 1rem;
    transition: border-color 0.3s ease;
  }
  .fancy-input-container input:focus {
    outline: none;
    border-color: #ffcc00;
  }
  .fancy-input-container .custom-dropdown {
    position: relative;
    margin: 5px;
    cursor: pointer;
  }
  .fancy-input-container .dropdown-button {
    padding: 10px;
    border: 2px solid #3d3d3d;
    border-radius: 10px;
    background-color: #e8e8e8;
    font-size: 1rem;
    transition: border-color 0.3s ease;
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: auto;
  }
  .fancy-input-container .dropdown-button:hover,
  .fancy-input-container .dropdown-button:focus {
    outline: none;
    border-color: #ffcc00;
  }
  .fancy-input-container .dropdown-menu {
    position: absolute;
    top: 100%;
    left: 0;
    background-color: #e8e8e8;
    width: auto;
    border: 2px solid #3d3d3d;
    border-radius: 10px;
    z-index: 1000;
    max-height: 200px;
    overflow-y: auto;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    display: block !important;
  }
  .fancy-input-container .dropdown-menu div {
    padding: 10px;
    cursor: pointer;
  }
  .fancy-input-container .dropdown-menu div:hover {
    background-color: #ffcc00;
  }
  .fancy-input-container .output {
    margin-top: 20px;
    font-size: 1.2rem;
    font-weight: bold;
    text-align: center;
    color: #3d3d3d;
  }
  .fancy-input-container .dropdown-arrow {
    margin-left: 10px;
    transition: transform 0.3s ease;
  }
  .fancy-input-container .dropdown-arrow.open {
    transform: rotate(180deg);
  }
</style>

<div class="fancy-input-container">
  <div class="fancy-input mx-auto">
    <input type="text" bind:value={inputValue} placeholder="Enter your input..." />
    <div class="custom-dropdown">
      <div class="dropdown-button" on:click={toggleDropdown}>
        <span>{selectedDomain || 'Select Domain'}</span>
        <span class="dropdown-arrow" class:open={dropdownOpen}>▼</span>
      </div>
      {#if dropdownOpen}
        <div class="dropdown-menu">
          {#each domains as domain}
            <div on:click={() => selectDomain(domain)}>{domain}</div>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  {#if selectedDomain && inputValue}
    <div class="output">
      {selectedDomain}/{inputValue}
    </div>
  {/if}
</div>