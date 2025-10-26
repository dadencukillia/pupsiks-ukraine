<script lang="ts">
  const {
    children = () => {},
    show = false,
    onclick = () => {},
    transparent = true
  }: {
    children: () => any,
    show: boolean|undefined,
    onclick: () => void|undefined,
    transparent: boolean|undefined
  } = $props();

  const onModalClick = (ev: MouseEvent) => {
    if (ev.currentTarget === ev.target) {
      ev.preventDefault();
      onclick();
    }
  };
</script>

{#if show}
  <div class={["fixed w-full h-full left-0 top-0 flex flex-column items-start z-100", transparent?"":"bg-[#FFF5]"].join(" ")} onclick={ onModalClick }>
    {@render children?.()}
</div>
{/if}
