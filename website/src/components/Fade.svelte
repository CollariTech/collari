<!-- ainda vou pensar na necessidade disso e transportar as animações pra dentro dos headers  -->
<script>
    import { fade } from 'svelte/transition';
    export let delay = 0;
  </script>
  
  <div transition:fade={{ delay }}>
    <slot></slot>
  </div>