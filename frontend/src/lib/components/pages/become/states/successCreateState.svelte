<script lang="ts">
  const {
    createdId
  }: {
    createdId: string
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">Успіх!</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">Сертифікат було створено!</p>
<a class="button w-fit" href={"/cert/" + createdId}>Поглянути</a>
