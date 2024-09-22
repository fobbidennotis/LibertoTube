<script lang="ts">
  export let resource: string;
  export let status: string;
</script>


<div class="row">
  <div class="col border-end">
    {resource}
  </div>
  <div class="col">
    <span class="{status === 'down' ? 'text-danger' : 'text-success'}">{status}</span>
  </div>
</div>
