<script lang="ts">
  const {
    color = "#FFF"
  }: {
    color: string|undefined
  } = $props();
</script>

<div class="loader-wrapper w-[76px] h-[38px]">
  <div class="loader" style={`--loader_color:${color};`}></div>
</div>

<style>
  .loader {
    color: var(--loader_color);
    width: 4px;
    aspect-ratio: 1;
    border-radius: 50%;
    box-shadow: 19px 0 0 7px, 38px 0 0 3px, 57px 0 0 0;
    animation: l21 .5s infinite alternate linear;
    transform: translateY(19px);
  }

  @keyframes l21 {
    50%  {box-shadow: 19px 0 0 3px, 38px 0 0 7px, 57px 0 0 3px}
    100% {box-shadow: 19px 0 0 0  , 38px 0 0 3px, 57px 0 0 7px}
  }
</style>
