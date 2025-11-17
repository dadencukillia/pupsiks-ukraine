<script lang="ts">
  import heroImgSm from "$lib/assets/heroSmall.jpg";
  import heroImgMd from "$lib/assets/heroMedium.jpg";
  import heroImgLg from "$lib/assets/heroLarge.jpg";

  const {
    showGetCertPopup
  }: {
    showGetCertPopup: () => void
  } = $props();
</script>

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
    <button class="button" onclick={ showGetCertPopup }>Перевірити пупсіка</button>
  </div>
</header>


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
