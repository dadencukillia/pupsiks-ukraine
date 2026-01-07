<script lang="ts">
	import { page } from '$app/state';
  import ErrorCard from "$lib/components/pages/certInfo/errorCard.svelte";
</script>

<svelte:head>
  <title>Сертифікат не знайдено — Асоціація пупсіків України</title>
  <meta property="og:title" content="Сертифікат не знайдено — Асоціація пупсіків України" />
  <meta property="og:type" content="website" />
</svelte:head>

<main class="min-w-full flex flex-col gap-3 sm:px-20 px-3 pb-10 text-center box-border">
  <ErrorCard errorText={ page.error?.message??"Невідома помилка" } />
</main>
