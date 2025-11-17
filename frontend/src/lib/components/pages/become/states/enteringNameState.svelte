<script lang="ts">
  let {
    goNext,
    goBack,
    input = $bindable(),
    nameValue = $bindable()
  }: {
    goNext: () => void,
    goBack: () => void,
    input: HTMLInputElement|null,
    nameValue: string
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">Введіть ПІБ</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">ПІБ потрібен, щоб відобразити на сертифікаті.</p>
<form onsubmit={ e => { e.preventDefault(); goNext(); } }>
  <input class="input mb-2" type="text" placeholder="Григорій Мельник Сергійович" required minlength="2" maxlength="200" bind:this={ input } bind:value={ nameValue }>
  <div class="flex flex-row">
    <button class="button-primary text-gray-200" type="button" onclick={ () => goBack() }>Назад</button>
    <button class="button" type="submit">Далі</button>
  </div>
</form>
