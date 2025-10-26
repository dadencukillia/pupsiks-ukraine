<script lang="ts">
  import { slide } from "svelte/transition";

  import heroImgSm from "$lib/assets/hero-small.jpg";
  import heroImgMd from "$lib/assets/hero-medium.jpg";
  import heroImgLg from "$lib/assets/hero-large.jpg";

  import FeatureCard from "$lib/components/pages/main/featureCard.svelte";
  import Modals from "$lib/components/modals.svelte";
  import GetCert from "$lib/components/modals/getCert.svelte";

  import "$lib/api_variables";

  let isGetCertModalShown: boolean = $state(false);

  const dismissModal = () => {
    isGetCertModalShown = false;
  };
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
  <header 
    style={`--h-sm:url(${JSON.stringify(heroImgSm)});--h-md:url(${JSON.stringify(heroImgMd)});--h-lg:url(${JSON.stringify(heroImgLg)});`}
    class="h-[300px] mb-12 bg-center bg-cover rounded-xl"
  >
    <div class="w-full h-full flex flex-col gap-2 justify-center items-center backdrop-blur-sm rounded-xl">
      <a class="button-primary flex flex-row items-center gap-1" href="/become">
        <span>Стати пупсіком вже сьогодні</span>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-right" viewBox="0 0 16 16">
          <path fill-rule="evenodd" d="M1 8a.5.5 0 0 1 .5-.5h11.793l-3.147-3.146a.5.5 0 0 1 .708-.708l4 4a.5.5 0 0 1 0 .708l-4 4a.5.5 0 0 1-.708-.708L13.293 8.5H1.5A.5.5 0 0 1 1 8"/>
        </svg>
      </a>
      <button class="button" onclick={ () => { isGetCertModalShown = true; } }>Перевірити пупсіка</button>
    </div>
  </header>

  <section class="mb-10">
    <h1 class="font-unbounded mb-3">Що це?</h1>
    <p>Асоціація пупсіків України (АПУ) — портал Всеукраїнської громадської організації "Асоціація Пупсиків України". Наша місія — визначити всіх пупсіків України.</p>
  </section>

  <section>
    <h1 class="font-unbounded mb-3">Чому слід вибрати нас?</h1>
    <p class="mb-2">АПУ єдина в своєму роді і має багато ключових переваг. Ось деякі з них:</p>
    <div class="w-full flex flex-col sm:flex-row gap-2">
      <FeatureCard title="⏳ Швидко" text="Отримуйте сертифікат менше ніж за 5 хвилин." />
      <FeatureCard title="💽 Велика база" text="Наша база вже має 0 зареєстрованих пупсіків!" />
      <FeatureCard title="🔒 Надійно" text="Дані з бази зникнуть лише якщо станеться щось погане!" />
    </div>
  </section>
</main>

<style>
  header {
    background-image: var(--h-sm);
  }

  @media (min-width: calc(640px + 100px)) {
    header {
      background-image: var(--h-md);
    }
  }

  @media (min-width: calc(1920px + 100px)) {
    header {
      background-image: var(--h-lg);
    }
  }
</style>
