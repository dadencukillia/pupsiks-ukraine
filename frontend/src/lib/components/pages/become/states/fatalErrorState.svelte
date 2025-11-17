<script lang="ts">
  const {
    errorTitle
  }: {
    errorTitle: string
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">{ errorTitle }</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">Перезапустіть сторінку і повторіть спробу пізніше.</p>
