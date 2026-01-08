<script lang="ts">
    import type { Snippet } from "svelte";

  const {
    children,
    label,
    onclick
  }: {
    children: Snippet,
    label: string,
    onclick: () => void
  } = $props();
</script>

<button class="button p-1 md:p-2 aspect-square md:aspect-auto text-brand-primary flex flex-row items-center gap-3" onclick={ onclick }>
  {@render children?.()}
  <span class="hidden md:inline text-black">{ label }</span>
</button>
