<script lang="ts">
  export let resource: string;
  export let status: string;
</script>

<div class="col">
  {resource} | <span class="{status === 'down' ? 'text-danger' : 'text-success'}">{status}</span>
</div>
