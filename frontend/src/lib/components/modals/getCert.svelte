<script lang="ts">
  let code = $state("");
</script>

<div class="w-full bg-brand-primary p-3">
  <div class="flex flex-col gap-2">
    <h1 class="font-unbounded text-white">Введіть серійний номер сертифікату</h1>
    <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();}}>
      <input class="input" type="text" placeholder="Серійний номер" required minlength="9" maxlength="9" bind:value={code}>
      <button class="button-primary" type="submit">Перевірити сертифікат</button>
    </form>
  </div>
</div>
