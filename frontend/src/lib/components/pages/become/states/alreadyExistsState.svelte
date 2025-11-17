<script lang="ts">
  const {
    goToForgotCert
  }: {
    goToForgotCert: () => void
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">Вже зайнято!</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">На вказану пошту вже прив'язаний сертифікат.</p>
<button class="button" onclick={ goToForgotCert }>Відправити сертифікат на пошту</button>
