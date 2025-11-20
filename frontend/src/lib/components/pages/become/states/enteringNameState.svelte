<script lang="ts">
  import { smartTrim } from "$lib/utils/smartTrim";
  import { tick } from "svelte";

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

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();
    const form = event.target as HTMLFormElement;

    // Ensures the DOM has been fully updated with the cleaned `nameValue`
    nameValue = smartTrim(nameValue);

    // Wait for DOM update
    await tick();

    // Validate cleaned value and go next
    if (form.reportValidity()) goNext();
  }
</script>

<h2 class="mb-1 font-bold text-white">Введіть ПІБ</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">ПІБ потрібен, щоб відобразити на сертифікаті.</p>
<form onsubmit={ onSubmit }>
  <input class="input mb-2" type="text" placeholder="Григорій Мельник Сергійович" required minlength="2" maxlength="200" bind:this={ input } bind:value={ nameValue }>
  <div class="flex flex-row">
    <button class="button-primary text-gray-200" type="button" onclick={ () => goBack() }>Назад</button>
    <button class="button" type="submit">Далі</button>
  </div>
</form>
