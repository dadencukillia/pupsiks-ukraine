<script lang="ts">
	import '../app.css';
  import favicon from '$lib/assets/favicon.ico';
	
	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Unbounded:wght@200..900&display=swap" rel="stylesheet">
</svelte:head>

<div class="navbar p-3 mb-10 flex gap-2 items-center justify-center">
  <a href={"/"} class="flex gap-2 items-center">
    <img src={favicon} alt="APU Logo" class="h-10">
    <span class="font-unbounded font-bold text-brand-primary">Асоціація пупсіків України</span>
  </a>
</div>

{@render children?.()}
