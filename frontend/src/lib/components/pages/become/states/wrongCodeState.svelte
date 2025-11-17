<script lang="ts">
  const {
    back
  }: {
    back: () => void
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">Хибний код!</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">Будьте обачні, у Вас обмежена кількість спроб.</p>
<button class="button-primary text-gray-200" type="button" onclick={ back }>Назад</button>
