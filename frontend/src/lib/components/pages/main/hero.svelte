<script lang="ts">
  import heroImg from "$lib/assets/hero.jpg?enhanced&w=400;800;1200;1600;2000;2400&quality=40";

  const {
    showGetCertPopup
  }: {
    showGetCertPopup: () => void
  } = $props();
</script>

<header class="relative h-[300px] mb-12">
  <enhanced:img
    src={ heroImg } alt="Hero Duck"
    class="absolute left-0 top-0 w-full h-full object-cover object-center rounded-xl -z-1"
    sizes="(min-width:640px) calc(100vw - 10rem), calc(100vw - 1.5rem)"
    fetchpriority="high" loading="eager"
  />
  <div class="w-full h-full flex flex-col gap-2 justify-center items-center backdrop-blur-sm rounded-xl">
    <a class="button-primary flex flex-row items-center gap-1" href="/become">
      <span>Стати пупсіком вже сьогодні</span>
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-right" viewBox="0 0 16 16">
        <path fill-rule="evenodd" d="M1 8a.5.5 0 0 1 .5-.5h11.793l-3.147-3.146a.5.5 0 0 1 .708-.708l4 4a.5.5 0 0 1 0 .708l-4 4a.5.5 0 0 1-.708-.708L13.293 8.5H1.5A.5.5 0 0 1 1 8"/>
      </svg>
    </a>
    <button class="button" onclick={ showGetCertPopup }>Перевірити сертифікат</button>
  </div>
</header>
