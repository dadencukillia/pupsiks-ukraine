<script lang="ts">
	import '../app.css';
  import Navbar from '$lib/components/pages/layout/navbar.svelte';
  import '@fontsource/unbounded/400.css';
  import '@fontsource/unbounded/700.css';
  import '@fontsource/unbounded/cyrillic-400.css';
  import '@fontsource/unbounded/cyrillic-700.css';
  import '@fontsource/unbounded/vietnamese-400.css';
  import '@fontsource/unbounded/vietnamese-700.css';

	let { children } = $props();
</script>

<Navbar />

{@render children?.()}
