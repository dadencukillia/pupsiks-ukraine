<script lang="ts">
  import { slide } from "svelte/transition";
  import Modals from "$lib/components/modals.svelte";
  import DeleteCertModal from "$lib/components/modals/deleteCert.svelte";
  import { onMount } from "svelte";
  import { getCert } from "$lib/api/requests/cert_crud.js";
  import Skeleton from "$lib/components/pages/certInfo/skeleton.svelte";
  import ErrorCard from "$lib/components/pages/certInfo/errorCard.svelte";
  import CertCard from "$lib/components/pages/certInfo/certCard.svelte";
  import Sidebar from "$lib/components/pages/certInfo/sidebar.svelte";

  const {
    data
  } = $props();

  let certInfo: {
    name: string,
    title: string,
  }|null = $state(null);

  let errorText: string|null = $state(null);

  // Modals
  let areModalsDismissable = $state(true);
  let isDeleteCertModalShown: boolean = $state(false);
  const dismissModal = () => {
    if (!areModalsDismissable) return;

    if (isDeleteCertModalShown) return isDeleteCertModalShown = false;
  };

  // Buttons event handlers
  const shareTelegramHandler = () => {
    window.open(`https://t.me/share/url?url=${encodeURIComponent(window.location.href)}&text=${encodeURIComponent("Я офіційно став пупсіком! Ось мій сертифікат ⬆️")}`, "_blank")?.focus();
  };

  const deleteCertHandler = () => {
    isDeleteCertModalShown = true;
  };

  // API
  onMount(() => {
    getCert(data.certId, {
      onSuccess: (data) => {
        certInfo = {
          name: data.name,
          title: data.title
        }
      },
      onError: (matcher, _message, _data) => {
        matcher.match({
          RESOURCE_NOT_FOUND: () => { errorText = "Сертифікат не знайдено" },
          BAD_REQUEST: () => { errorText = "Неправильний серійний номер" },
          FATAL_ERROR: () => { errorText = "Помилка з'єднання" },
          default: () => { errorText = "Невідома помилка" }
        });
      }
    });
  });
</script>

<svelte:head>
  {#if certInfo}
    <title>{certInfo.name} — Асоціація пупсіків України</title>
  {:else}
    <title>Сертифікат пупсіка — Асоціація пупсіків України</title>
  {/if}
</svelte:head>

<Modals show={ isDeleteCertModalShown } transparent={ false } onclick={ dismissModal }>
  {#if certInfo && isDeleteCertModalShown}
    <DeleteCertModal 
      closePopup={ () => isDeleteCertModalShown = false } 
      setDismissable={ (value: boolean) => {areModalsDismissable = value} } 
      certId={ data.certId }
    />
  {/if}
</Modals>

<main class="sm:px-20 px-3 pb-10" transition:slide>
  {#if errorText !== null}
    <ErrorCard errorText={ errorText } />
  {:else if certInfo}
    <h1 class="font-unbounded mb-10">Сертифікат пупсіка</h1>
    <div class="w-full flex flex-col md:flex-row gap-12 justify-between">
      <CertCard certName={ certInfo.name } certTitle={ certInfo.title } certId={ data.certId } />
      <Sidebar 
        shareTelegramHandler={ shareTelegramHandler }
        deleteCertHandler={ deleteCertHandler }
      />
    </div>
  {:else}
    <Skeleton />
  {/if}
</main>
