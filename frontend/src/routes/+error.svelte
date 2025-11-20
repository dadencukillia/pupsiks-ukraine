<script lang="ts">
  import banner from "$lib/assets/404banner.svg";
	import { page } from '$app/state';
</script>

<main class="min-w-full flex flex-col gap-3 sm:px-20 px-3 pb-10 text-center box-border">
  {#if page.status === 404}
    <img src={banner} alt="404 banner">
  {:else}
    <h1 class="font-bold text-9xl">{page.status}</h1>
  {/if}
</main>
