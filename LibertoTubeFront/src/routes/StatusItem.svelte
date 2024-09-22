<script lang="ts">
  export let resource: string;
  export let status: string;
 </script>
 
 <style>
  .status-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    border-bottom: 1px solid #eaeaea;
    font-size: clamp(0.9rem, 1.8vw, 1.3rem);
  }
 
  .status-item:last-child {
    border-bottom: none;
  }
 
  .resource {
    flex: 1;
    padding-right: 10px;
    word-break: break-all;
  }
 
  .status {
    font-weight: bold;
  }
 
  .status.down {
    color: #ff4136;
  }
 
  .status.available {
    color: #2ecc40;
  }
 
  @media (max-width: 480px) {
    .status-item {
      flex-direction: column;
      align-items: flex-start;
    }
 
    .resource {
      margin-bottom: 5px;
    }
  }
 </style>
 
 <div class="status-item">
  <div class="resource">{resource}</div>
  <div class="status {status}">{status}</div>
 </div>