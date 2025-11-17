<script lang="ts">
  import { slide } from "svelte/transition";
  import Modals from "$lib/components/modals.svelte";
  import GetCert from "$lib/components/modals/getCert.svelte";
  import { onMount } from "svelte";
  import { getUsersCount } from "$lib/api/requests/stats";
  import Hero from "$lib/components/pages/main/hero.svelte";
  import AboutSection from "$lib/components/pages/main/aboutSection.svelte";
  import FeaturesSection from "$lib/components/pages/main/featuresSection.svelte";

  let usersCount = $state("...");

  // Modals
  let isGetCertModalShown: boolean = $state(false);

  // Buttons event handlers
  const dismissModal = () => {
    isGetCertModalShown = false;
  };

  // API
  onMount(() => {
    getUsersCount({
      onSuccess: (data) => {
          usersCount = data.count + "";
      },
      onError: (codeError, message, _data) => {
        console.error(codeError, message);
      },
      onFatal: (error) => {
        console.error(error);
      }
    });
  });
</script>

<svelte:head>
  <title>Головна — Асоціація пупсіків України</title>
</svelte:head>

<Modals show={ isGetCertModalShown } transparent={ false } onclick={ dismissModal }>
  {#if isGetCertModalShown}
    <GetCert />
  {/if}
</Modals>

<main class="sm:px-20 px-3 pb-10" transition:slide>
  <Hero showGetCertPopup={ () => { isGetCertModalShown = true; } } />
  <AboutSection />
  <FeaturesSection usersCount={ usersCount } />
</main>
