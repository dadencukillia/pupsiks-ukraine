<script lang="ts">
  import { onDestroy } from "svelte";
  import Loader from "../loader.svelte";
  import { goto } from "$app/navigation";

  const {
    closePopup,
    setDismissable,
  }: {
    closePopup: () => void,
    setDismissable: (value: boolean) => void,
  } = $props();

  const StateEnum = {
    ApprovingDeletion: 0,
    EnteringEmail: 1,
    SendingEmailLoader: 2,
    EnteringCode: 3,
    CheckingCodeLoader: 4,
    Success: 5,
    WrongEmail: 6,
    WrongCode: 7,
  };

  setDismissable(true);

  let currentState: number = $state(StateEnum.ApprovingDeletion);
  let email: string = $state("");
  let code: string = $state("");
  let timer: number = $state(10);

  let timerDecreaseInterval: number|null = null;

  const approveDeletion = () => {
    currentState = StateEnum.EnteringEmail;
  };

  const sendEmailCode = async () => {
    currentState = StateEnum.SendingEmailLoader;
    currentState = StateEnum.EnteringCode;
  };

  const deleteCert = async () => {
    currentState = StateEnum.CheckingCodeLoader;
    currentState = StateEnum.Success;

    timerDecreaseInterval = setInterval(() => {
      timer--;
      if (timer <= 0 && timerDecreaseInterval) {
        clearInterval(timerDecreaseInterval);
        timerDecreaseInterval = null;
        goto("/");
      }
    }, 1000);

    setDismissable(false);
  };

  onDestroy(() => {
    if (timerDecreaseInterval) {
      clearInterval(timerDecreaseInterval);
      timerDecreaseInterval = null;
    }
  });
</script>

<div class="w-full bg-brand-primary p-3">
  {#if currentState === StateEnum.ApprovingDeletion}
    <div class="flex flex-row items-center gap-2">
      <h1 class="flex-1 font-unbounded text-white">Ви дійсно хочете видалити сертифікат?</h1>
      <div class="flex flex-row gap-2">
        <button class="button-primary" onclick={ approveDeletion }>Так</button>
        <button class="button px-10" onclick={ closePopup }>Ні</button>
      </div>
    </div>
  {:else if currentState === StateEnum.EnteringEmail}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Введіть вашу електронну пошту</h1>
      <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();sendEmailCode();}}>
        <input class="input" type="email" placeholder="me@example.com" required bind:value={email}>
        <button class="button-primary" type="submit">Відправити код</button>
      </form>
    </div>
  {:else if currentState === StateEnum.SendingEmailLoader || currentState === StateEnum.CheckingCodeLoader}
    <Loader />
  {:else if currentState === StateEnum.EnteringCode}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Введіть код з пошти</h1>
      <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();deleteCert();}}>
        <input class="input" type="text" placeholder="AAA111BBB" required minlength="9" maxlength="9" bind:value={code}>
        <button class="button-primary" type="submit">Видалити сертифікат</button>
      </form>
    </div>
  {:else if currentState === StateEnum.Success}
    <h1 class="font-unbounded text-white">Успішно видалено!</h1>
    <p class="text-white">Перенаправлення через ({ timer } с.)</p>
  {:else if currentState === StateEnum.WrongEmail}
    <div class="flex flex-row gap-2">
      <div class="flex-1 flex-col">
        <h1 class="font-unbounded text-white">Отакої!</h1>
        <p class="text-white">Сертифікат не належить пошті яку Ви ввели!</p>
      </div>
      <div class="flex flex-row items-center">
        <button class="button" onclick={() => closePopup()}>Зрозумів</button>
      </div>
    </div>
  {:else if currentState === StateEnum.WrongCode}
    <div class="flex flex-row gap-2">
      <div class="flex-1 flex-col">
        <h1 class="font-unbounded text-white">Отакої!</h1>
        <p class="text-white">Ви ввели хибний код!</p>
      </div>
      <div class="flex flex-row items-center">
        <button class="button" onclick={() => closePopup()}>Зрозумів</button>
      </div>
    </div>
  {/if}
</div>
