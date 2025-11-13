<script lang="ts">
  import { onDestroy } from "svelte";
  import Loader from "../loader.svelte";
  import { goto } from "$app/navigation";
  import { durationFormat } from "$lib/utils/timeFormat";
  import { sendCodeCertDeletion } from "$lib/api/requests/code_confirmation";
  import { ERROR_BAD_REQUEST, ERROR_EMAIL_RATE_LIMIT, ERROR_INTERNAL_SERVER_ERROR, ERROR_INVALID_CODE, ERROR_INVALID_EMAIL, ERROR_IP_RATE_LIMIT, ERROR_RESOURCE_NOT_FOUND, ERROR_TRIES_OUT } from "$lib/api/configs";
  import { deleteCert } from "$lib/api/requests/cert_crud";
  import { EMAIL_CODE_PATTERN } from "$lib/api/regexPatterns";

  const {
    closePopup,
    setDismissable,
    certId
  }: {
    closePopup: () => void,
    setDismissable: (value: boolean) => void,
    certId: string
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
    CodeRateLimit: 8,
    FatalError: 9,
    TriesOut: 10
  };

  setDismissable(true);

  let currentState: number = $state(StateEnum.ApprovingDeletion);
  let email: string = $state("");
  let code: string = $state("");
  let emailConfirmationToken = "";
  let timerSeconds: number = $state(0);

  let timerDecreaseInterval: number|undefined = undefined;

  const approveDeletion = () => {
    currentState = StateEnum.EnteringEmail;
  };

  const submitEmail = async () => {
    currentState = StateEnum.SendingEmailLoader;

    await sendCodeCertDeletion(email, certId, {
      onSuccess: (data) => {
        emailConfirmationToken = data.token;
        currentState = StateEnum.EnteringCode;
      },
      onError: (codeError, message, data) => {
        console.error(codeError, message);

        const setRateLimitInterval = (rateTimestamp: number) => {
          clearInterval(timerDecreaseInterval);

          timerDecreaseInterval = setInterval(() => {
            const currentTimestamp = Math.ceil(Date.now() / 1000);
            timerSeconds = rateTimestamp - currentTimestamp;

            if (timerSeconds < 0) {
              currentState = StateEnum.EnteringEmail;
              clearInterval(timerDecreaseInterval);
            }
          }, 1000);
        };

        if (
          codeError === ERROR_EMAIL_RATE_LIMIT ||
          codeError === ERROR_IP_RATE_LIMIT
        ) {
          currentState = StateEnum.CodeRateLimit;
          const currentTimestamp = Math.ceil(Date.now() / 1000);
          let rateTimestamp = data["timestamp"];
          timerSeconds = rateTimestamp - currentTimestamp;
          setRateLimitInterval(rateTimestamp);
        } else if (codeError === ERROR_INVALID_EMAIL) {
          currentState = StateEnum.WrongEmail;
        } else if (codeError === ERROR_RESOURCE_NOT_FOUND) {
          currentState = StateEnum.FatalError;
        } else if (codeError === ERROR_BAD_REQUEST) {
          currentState = StateEnum.FatalError;
        } else if (codeError === ERROR_INTERNAL_SERVER_ERROR) {
          currentState = StateEnum.FatalError;
        } else {
          currentState = StateEnum.FatalError;
        }
      },
      onFatal: (error) => {
        console.error(error);

        currentState = StateEnum.FatalError;
      }
    });
  };

  const submitCode = async () => {
    currentState = StateEnum.CheckingCodeLoader;

    await deleteCert({
      email: email,
      code: code,
      token: emailConfirmationToken
    }, {
      onSuccess: (data) => {
        currentState = StateEnum.Success;

        clearInterval(timerDecreaseInterval);
        timerSeconds = 10;
        timerDecreaseInterval = setInterval(() => {
          if (--timerSeconds <= 0) {
            clearInterval(timerDecreaseInterval);
            goto("/");
          }
        }, 1000);

        setDismissable(false);
      },
      onError: (codeError, message, data) => {
        console.error(codeError, message);

        if (codeError === ERROR_INVALID_CODE) {
          currentState = StateEnum.WrongCode;
        } else if (codeError === ERROR_TRIES_OUT) {
          currentState = StateEnum.TriesOut;

          const currentTimestamp = Math.ceil(Date.now() / 1000);
          const timerTimestamp = data["timestamp"];
          timerSeconds = timerTimestamp - currentTimestamp;

          clearInterval(timerDecreaseInterval);
          timerDecreaseInterval = setInterval(() => {
            const currentTimestamp = Math.ceil(Date.now() / 1000);
            timerSeconds = timerTimestamp - currentTimestamp;

            if (timerSeconds < 0) {
              currentState = StateEnum.EnteringEmail;
              clearInterval(timerDecreaseInterval);
            }
          }, 1000);
        } else if (codeError === ERROR_BAD_REQUEST) {
          currentState = StateEnum.FatalError;
        } else {
          currentState = StateEnum.FatalError;
        }
      },
      onFatal: (error) => {
        console.error(error);

        currentState = StateEnum.FatalError;
      },
    });
  };

  onDestroy(() => {
    clearInterval(timerDecreaseInterval);
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
      <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();submitEmail();}}>
        <input class="input" type="email" placeholder="me@example.com" required bind:value={email}>
        <button class="button-primary" type="submit">Відправити код</button>
      </form>
    </div>
  {:else if currentState === StateEnum.SendingEmailLoader || currentState === StateEnum.CheckingCodeLoader}
    <Loader />
  {:else if currentState === StateEnum.EnteringCode}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Введіть код з пошти</h1>
      <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();submitCode();}}>
        <input class="input uppercase" type="text" placeholder="AAA111BBB" required minlength="9" maxlength="9" pattern={EMAIL_CODE_PATTERN} bind:value={code}>
        <button class="button-primary" type="submit">Видалити сертифікат</button>
      </form>
    </div>
  {:else if currentState === StateEnum.Success}
    <h1 class="font-unbounded text-white">Успішно видалено!</h1>
    <p class="text-white">Перенаправлення через ({ timerSeconds } с.)</p>
  {:else if currentState === StateEnum.WrongEmail}
    <div class="flex flex-row gap-2">
      <div class="flex-1 flex-col">
        <h1 class="font-unbounded text-white">Отакої!</h1>
        <p class="text-white">Сертифікат не належить пошті яку Ви ввели!</p>
      </div>
      <div class="flex flex-row items-center">
        <button class="button" onclick={ () => closePopup() }>Зрозумів</button>
      </div>
    </div>
  {:else if currentState === StateEnum.WrongCode}
    <div class="flex flex-row gap-2">
      <div class="flex-1 flex-col">
        <h1 class="font-unbounded text-white">Хибний код!</h1>
        <p class="text-white">Будьте обачні, у Вас обмежена кількість спроб.</p>
      </div>
      <div class="flex flex-row items-center">
        <button class="button" onclick={ () => { currentState = StateEnum.EnteringCode; } }>Назад</button>
      </div>
    </div>
  {:else if currentState === StateEnum.CodeRateLimit}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Почекайте, щоб відправити код знову!</h1>
      <p class="text-white">{ durationFormat(timerSeconds) }</p>
    </div>
  {:else if currentState === StateEnum.FatalError}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Невідома помилка</h1>
      <p class="text-white">Перезапустіть сторінку і повторіть спробу пізніше.</p>
    </div>
  {:else if currentState === StateEnum.TriesOut}
    <div class="flex flex-col gap-2">
      <h1 class="font-unbounded text-white">Спроби закінчилися!</h1>
      <p class="text-white">{ durationFormat(timerSeconds) }</p>
    </div>
  {/if}
</div>
