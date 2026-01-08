<script lang="ts">
  import { slide } from "svelte/transition";
  import Modals from "$lib/components/modals.svelte";
  import DeleteCertModal from "$lib/components/modals/deleteCert.svelte";
  import CertCard from "$lib/components/pages/certInfo/certCard.svelte";
  import Sidebar from "$lib/components/pages/certInfo/sidebar.svelte";
  import SidebarButton from "$lib/components/pages/certInfo/sidebarButton.svelte";
  import { downloadFile } from "$lib/utils/downloadFile";

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

  const ogUrl = `${data.websiteOrigin}/cert/${data.cert.id}/og.png`;

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

  const downloadImageHandler = () => {
    downloadFile(ogUrl, `${data.cert.id}.png`);
  };
</script>


<svelte:head>
  <title>{data.cert.name} — Асоціація пупсіків України</title>
  <meta name="description" content={ `Сертифікат почесного пупсіка ${data.cert.name}. Доєднуйтеся і Ви!` } />
  <meta property="og:title" content={ `${data.cert.name} — Асоціація пупсіків України` } />
  <meta property="og:type" content="website" />
  <meta property="og:image" content={ ogUrl } />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:image" content={ ogUrl } />
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
    <Sidebar>
      <SidebarButton label="Поділитися Telegram" onclick={ shareTelegramHandler }>
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-telegram" viewBox="0 0 16 16">
          <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0M8.287 5.906q-1.168.486-4.666 2.01-.567.225-.595.442c-.03.243.275.339.69.47l.175.055c.408.133.958.288 1.243.294q.39.01.868-.32 3.269-2.206 3.374-2.23c.05-.012.12-.026.166.016s.042.12.037.141c-.03.129-1.227 1.241-1.846 1.817-.193.18-.33.307-.358.336a8 8 0 0 1-.188.186c-.38.366-.664.64.015 1.088.327.216.589.393.85.571.284.194.568.387.936.629q.14.092.27.187c.331.236.63.448.997.414.214-.02.435-.22.547-.82.265-1.417.786-4.486.906-5.751a1.4 1.4 0 0 0-.013-.315.34.34 0 0 0-.114-.217.53.53 0 0 0-.31-.093c-.3.005-.763.166-2.984 1.09"/>
        </svg>
      </SidebarButton>
      <SidebarButton label="Видалити сертифікат" onclick={ deleteCertHandler }>
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-trash3-fill" viewBox="0 0 16 16">
          <path d="M11 1.5v1h3.5a.5.5 0 0 1 0 1h-.538l-.853 10.66A2 2 0 0 1 11.115 16h-6.23a2 2 0 0 1-1.994-1.84L2.038 3.5H1.5a.5.5 0 0 1 0-1H5v-1A1.5 1.5 0 0 1 6.5 0h3A1.5 1.5 0 0 1 11 1.5m-5 0v1h4v-1a.5.5 0 0 0-.5-.5h-3a.5.5 0 0 0-.5.5M4.5 5.029l.5 8.5a.5.5 0 1 0 .998-.06l-.5-8.5a.5.5 0 1 0-.998.06m6.53-.528a.5.5 0 0 0-.528.47l-.5 8.5a.5.5 0 0 0 .998.058l.5-8.5a.5.5 0 0 0-.47-.528M8 4.5a.5.5 0 0 0-.5.5v8.5a.5.5 0 0 0 1 0V5a.5.5 0 0 0-.5-.5"/>
        </svg>
      </SidebarButton>
      <SidebarButton label="Завантажити .png" onclick={ downloadImageHandler }>
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-image-fill" viewBox="0 0 16 16">
          <path d="M.002 3a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-12a2 2 0 0 1-2-2zm1 9v1a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V9.5l-3.777-1.947a.5.5 0 0 0-.577.093l-3.71 3.71-2.66-1.772a.5.5 0 0 0-.63.062zm5-6.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0"/>
        </svg>
      </SidebarButton>
    </Sidebar>
  </div>
</main>
