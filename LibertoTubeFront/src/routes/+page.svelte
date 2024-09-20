<script lang="ts">
  import StatusItem from './StatusItem.svelte';

  let proxy_status_data = {
    "gfbrowser.com": "availible",
    "down_example.com": "down"
  };
</script>

<div class="container mx-auto text-center" id="domains">
  <h2 class="mt-5">List of available alternative domains:</h2>
  <div class="container">
    <div class="col">
      {#each Object.entries(proxy_status_data) as [resource, status]}
        <StatusItem {resource} {status} />
      {/each}
    </div>
  </div>
</div>