<script lang="ts">
  import { slide } from "svelte/transition";
  import Modals from "$lib/components/modals.svelte";
  import DeleteCertModal from "$lib/components/modals/deleteCert.svelte";
  import CertCard from "$lib/components/pages/certInfo/certCard.svelte";
  import Sidebar from "$lib/components/pages/certInfo/sidebar.svelte";

  type Props = {
    data: {
      cert: {
        id: string,
        name: string,
        title: string
      },
      websiteOrigin: string
    }
  };

  const {
    data
  } = $props() as Props;

  // Modals
  let areModalsDismissable = $state(true);
  let isDeleteCertModalShown: boolean = $state(false);
  const dismissModal = () => {
    if (!areModalsDismissable) return;

    if (isDeleteCertModalShown) return isDeleteCertModalShown = false;
  };

  // Buttons event handlers
  const shareTelegramHandler = () => {
    window.open(`https://t.me/share/url?url=${encodeURIComponent(window.location.href)}&text=${encodeURIComponent("Я офіційно став(ла) пупсіком! Ось мій сертифікат ⬆️")}`, "_blank")?.focus();
  };

  const deleteCertHandler = () => {
    isDeleteCertModalShown = true;
  };
</script>


<svelte:head>
  <title>{data.cert.name} — Асоціація пупсіків України</title>
  <meta name="description" content={ `Сертифікат почесного пупсика ${data.cert.name}. Доєднуйтеся і Ви!` } />
  <meta property="og:title" content={ `${data.cert.name} — Асоціація пупсіків України` } />
  <meta property="og:type" content="website" />
  <meta property="og:image" content={ `${data.websiteOrigin}/cert/${data.cert.id}/og.png` } />
</svelte:head>

<Modals show={ isDeleteCertModalShown } transparent={ false } onclick={ dismissModal }>
  {#if isDeleteCertModalShown}
    <DeleteCertModal 
      closePopup={ () => isDeleteCertModalShown = false } 
      setDismissable={ (value: boolean) => {areModalsDismissable = value} } 
      certId={ data.cert.id }
    />
  {/if}
</Modals>

<main class="sm:px-20 px-3 pb-10" transition:slide>
  <h1 class="font-unbounded mb-10">Сертифікат пупсіка</h1>
  <div class="w-full flex flex-col md:flex-row gap-12 justify-between">
    <CertCard certName={ data.cert.name } certTitle={ data.cert.title } certId={ data.cert.id } />
    <Sidebar 
      shareTelegramHandler={ shareTelegramHandler }
      deleteCertHandler={ deleteCertHandler }
    />
  </div>
</main>
