<script lang="ts">
  import { EMAIL_CODE_PATTERN } from "$lib/api/regexPatterns";

  let {
    goNext,
    goBack,
    input = $bindable(),
    codeValue = $bindable()
  }: {
    goNext: () => void,
    goBack: () => void,
    input: HTMLInputElement|null,
    codeValue: string
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">Введіть код з пошти</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">Код потрібен, щоб підтвердити належність пошти.</p>
<form onsubmit={ e => { e.preventDefault(); goNext(); } }>
  <input class="input mb-2 uppercase" type="text" placeholder="AAA111BBB" required minlength="9" maxlength="9" pattern={ EMAIL_CODE_PATTERN } bind:this={ input } bind:value={ codeValue }>
  <div class="flex flex-row">
    <button class="button-primary text-gray-200" type="button" onclick={ () => goBack() }>Назад</button>
    <button class="button" type="submit">Отримати сертифікат</button>
  </div>
</form>
