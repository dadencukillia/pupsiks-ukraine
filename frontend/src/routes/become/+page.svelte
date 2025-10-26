<script lang="ts">
  import { slide } from "svelte/transition";

  const steps = 4;
  let step = $state<number>(0);

  let email: string = $state<string>("");
  let name: string = $state<string>("");
  let title: string = $state<string>("");
  let code = $state<string>("");

  let autofocusInputs: Array<HTMLInputElement|null> = [];

  $effect(() => {
    const input: HTMLInputElement|null = autofocusInputs[step];
    if (input) {
      input.focus();
    }
  });
</script>

<svelte:head>
  <title>Отримати сертифікат — Асоціація пупсіків України</title>
</svelte:head>

<main class="sm:px-20 px-3 pb-12 w-full bg-brand-primary" transition:slide>
  <h1 class="mb-2 font-unbounded text-white">Щоб стати пупсіком залишилося {steps-step} кроків!</h1>
  <div class="mb-5 flex flex-row gap-3 w-full items-center">
    {#each [...new Array(steps).keys()] as stepBar}
      {#if stepBar <= step}
        <div class="flex-1 rounded-full bg-white h-1"></div>
      {:else}
        <div class="flex-1 rounded-full bg-orange-300 h-1"></div>
      {/if}
    {/each}
  </div>

  {#if step == 0}
    <h2 class="mb-1 font-bold text-white">Введіть пошту</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Пошта потрібно, щоб ідентифікувати власника сертифіката. Завдяки власній пошті Ви зможете контролювати свій сертифікат у базі.</p>
    <form onsubmit={e => {e.preventDefault();step++;}}>
      <input class="input mb-2" type="email" placeholder="me@example.com" required bind:this={autofocusInputs[0]} bind:value={email}>
      <button class="button" type="submit">Далі</button>
    </form>
  {:else if step == 1}
    <h2 class="mb-1 font-bold text-white">Введіть ПІБ</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">ПІБ потрібен, щоб відобразити на сертифікаті.</p>
    <form onsubmit={e => {e.preventDefault();step++;}}>
      <input class="input mb-2" type="text" placeholder="Григорій Мельник Сергійович" required minlength="2" maxlength="200" bind:this={autofocusInputs[1]} bind:value={name}>
      <div class="flex flex-row">
        <button class="button-primary text-gray-200" type="button" onclick={() => step--}>Назад</button>
        <button class="button" type="submit">Далі</button>
      </div>
    </form>
  {:else if step == 2}
    <h2 class="mb-1 font-bold text-white">Придумайте собі титул</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Титул потрібен, щоб відобразити на сертифікаті.</p>
    <form onsubmit={e => {e.preventDefault();step++;}}>
      <input class="input mb-2" type="text" placeholder="Король пупсіків" required minlength="1" maxlength="200" bind:this={autofocusInputs[2]} bind:value={title}>
      <div class="flex flex-row">
        <button class="button-primary text-gray-200" type="button" onclick={() => step--}>Назад</button>
        <button class="button" type="submit">Далі</button>
      </div>
    </form>
  {:else if step == 3}
    <h2 class="mb-1 font-bold text-white">Введіть код з пошти</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Код потрібен, щоб підтвердити належність пошти.</p>
    <form onsubmit={e => {e.preventDefault();step++;}}>
      <input class="input mb-2" type="text" placeholder="AAA111BBB" required minlength="9" maxlength="9" bind:this={autofocusInputs[3]} bind:value={code}>
      <div class="flex flex-row">
        <button class="button-primary text-gray-200" type="button" onclick={() => step--}>Назад</button>
        <button class="button" type="submit">Отримати сертифікат</button>
      </div>
    </form>
  {/if}
</main>
