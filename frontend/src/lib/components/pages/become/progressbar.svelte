<script lang="ts">
  const {
    step,
    maxSteps
  }: {
    step: number,
    maxSteps: number
  } = $props();
</script>

{#if step >= 0}
  <h1 class="mb-2 font-unbounded text-white">Щоб стати пупсіком залишилося { Math.max(maxSteps - step, 0) } кроків!</h1>
  <div class="mb-5 flex flex-row gap-3 w-full items-center">
    {#each [...new Array(maxSteps).keys()] as stepBar}
      {#if stepBar < step}
        <div class="flex-1 rounded-full bg-white h-1"></div>
      {:else}
        <div class="flex-1 rounded-full bg-orange-300 h-1"></div>
      {/if}
    {/each}
  </div>
{/if}
