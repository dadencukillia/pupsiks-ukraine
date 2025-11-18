<script lang="ts">
  import { onDestroy } from "svelte";
  import Loader from "../loader.svelte";
  import { goto } from "$app/navigation";
  import { durationFormat } from "$lib/utils/timeFormat";
  import { sendCodeCertDeletion } from "$lib/api/requests/code_confirmation";
  import { deleteCert } from "$lib/api/requests/cert_crud";
  import { EMAIL_CODE_PATTERN } from "$lib/api/regexPatterns";
  import { FiniteStateMachine } from "svelte-state-machine";
  import { Timer } from "$lib/utils/reactiveTimer.svelte";

  // Properties
  const {
    closePopup,
    setDismissable,
    certId
  }: {
    closePopup: () => void,
    setDismissable: (value: boolean) => void,
    certId: string
  } = $props();

  // Timer
  const timer = new Timer();

  // States
  const FSM = new FiniteStateMachine(
    "ApprovingDeletion",
    "EnteringEmail",
    "SendingEmailLoader",
    "EnteringCode",
    "CheckingCodeLoader",
    "Success",
    "WrongEmail",
    "WrongCode",
    "CodeRateLimit",
    "FatalError",
    "TriesOut"
  );

  setDismissable(true);

  let email: string = $state("");
  let code: string = $state("");
  let emailConfirmationToken: string = "";

  // Buttons event handlers
  const approveDeletion = () => {
    FSM.state = FSM.enum.EnteringEmail;
  };

  const submitEmail = async () => {
    FSM.state = FSM.enum.SendingEmailLoader;

    await sendCodeCertDeletion(email, certId, {
      onSuccess: (data) => {
        emailConfirmationToken = data.token;
        FSM.state = FSM.enum.EnteringCode;
      },
      onError: (matcher, _message, data) => {
        const onRateLimit = () => {
          FSM.state = FSM.enum.CodeRateLimit;

          timer.onEnd = _ => {
            FSM.state = FSM.enum.EnteringEmail;
          };

          timer.runTimestampSeconds(data["timestamp"] as number);
        };

        matcher.match({
          EMAIL_RATE_LIMIT: onRateLimit,
          IP_RATE_LIMIT: onRateLimit,
          INVALID_EMAIL: () => { FSM.state = FSM.enum.WrongEmail },
          default: () => { FSM.state = FSM.enum.FatalError }
        });
      }
    });
  };

  const submitCode = async () => {
    FSM.state = FSM.enum.CheckingCodeLoader;

    await deleteCert({
      email: email,
      code: code,
      token: emailConfirmationToken
    }, {
      onSuccess: (_data) => {
        FSM.state = FSM.enum.Success;

        timer.onEnd = _ => {
          goto("/");
        };

        setDismissable(false);
        timer.runSeconds(10);
      },
      onError: (matcher, _message, data) => {
        matcher.match({
          INVALID_CODE: () => { FSM.state = FSM.enum.WrongCode },
          TRIES_OUT: () => {
            FSM.state = FSM.enum.TriesOut;

            timer.onEnd = _ => {
              FSM.state = FSM.enum.EnteringEmail;
            }

            timer.runTimestampSeconds(data["timestamp"] as number);
          },
          default: () => { FSM.state = FSM.enum.FatalError }
        });
      },
    });
  };

  onDestroy(() => {
    timer.stop(false);
  });
</script>

<div class="w-full bg-brand-primary p-3">
  {#if FSM.check.ApprovingDeletion()}
    <div class="flex flex-row items-center gap-2">
      <h1 class="flex-1 font-unbounded text-white">Ви дійсно хочете видалити сертифікат?</h1>
      <div class="flex flex-row gap-2">
        <button class="button-primary" onclick={ approveDeletion }>Так</button>
        <button class="button px-10" onclick={ closePopup }>Ні</button>
      </div>
    </div>
  {:else if FSM.check.EnteringEmail()}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Введіть вашу електронну пошту</h1>
      <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();submitEmail();}}>
        <input class="input" type="email" placeholder="me@example.com" required bind:value={email}>
        <button class="button-primary" type="submit">Відправити код</button>
      </form>
    </div>
  {:else if FSM.check.SendingEmailLoader() || FSM.check.CheckingCodeLoader()}
    <Loader />
  {:else if FSM.check.EnteringCode()}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Введіть код з пошти</h1>
      <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();submitCode();}}>
        <input class="input uppercase" type="text" placeholder="AAA111BBB" required minlength="9" maxlength="9" pattern={EMAIL_CODE_PATTERN} bind:value={code}>
        <button class="button-primary" type="submit">Видалити сертифікат</button>
      </form>
    </div>
  {:else if FSM.check.Success()}
    <h1 class="font-unbounded text-white">Успішно видалено!</h1>
    <p class="text-white">Перенаправлення через ({ timer.remainSeconds } с.)</p>
  {:else if FSM.check.WrongEmail()}
    <div class="flex flex-row gap-2">
      <div class="flex-1 flex-col">
        <h1 class="font-unbounded text-white">Отакої!</h1>
        <p class="text-white">Сертифікат не належить пошті яку Ви ввели!</p>
      </div>
      <div class="flex flex-row items-center">
        <button class="button" onclick={ () => closePopup() }>Зрозумів</button>
      </div>
    </div>
  {:else if FSM.check.WrongCode()}
    <div class="flex flex-row gap-2">
      <div class="flex-1 flex-col">
        <h1 class="font-unbounded text-white">Хибний код!</h1>
        <p class="text-white">Будьте обачні, у Вас обмежена кількість спроб.</p>
      </div>
      <div class="flex flex-row items-center">
        <button class="button" onclick={ () => { FSM.state = FSM.enum.EnteringCode; } }>Назад</button>
      </div>
    </div>
  {:else if FSM.check.CodeRateLimit()}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Почекайте, щоб відправити код знову!</h1>
      <p class="text-white">{ durationFormat(timer.remainSeconds) }</p>
    </div>
  {:else if FSM.check.FatalError()}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Невідома помилка</h1>
      <p class="text-white">Перезапустіть сторінку і повторіть спробу пізніше.</p>
    </div>
  {:else if FSM.check.TriesOut()}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Спроби закінчилися!</h1>
      <p class="text-white">{ durationFormat(timer.remainSeconds) }</p>
    </div>
  {/if}
</div>
