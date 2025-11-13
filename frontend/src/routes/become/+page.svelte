<script lang="ts">
  import { ERROR_ALREADY_EXISTS, ERROR_BAD_REQUEST, ERROR_EMAIL_RATE_LIMIT, ERROR_INTERNAL_SERVER_ERROR, ERROR_INVALID_CODE, ERROR_IP_RATE_LIMIT, ERROR_TRIES_OUT } from "$lib/api/configs";
  import { EMAIL_CODE_PATTERN } from "$lib/api/regexPatterns";
  import { createCert, forgotCert } from "$lib/api/requests/cert_crud";
  import { sendCodeCertCreation } from "$lib/api/requests/code_confirmation";
  import Loader from "$lib/components/loader.svelte";
  import { durationFormat } from "$lib/utils/timeFormat";
  import { onMount } from "svelte";
  import { slide } from "svelte/transition";

  const steps = 4;
  let step = $state<number>(0);

  let email: string = $state<string>("");
  let name: string = $state<string>("");
  let title: string = $state<string>("");
  let code = $state<string>("");

  const StateEnum = {
    EnteringEmail: 0,
    EnteringName: 1,
    EnteringTitle: 2,
    SendingEmailLoader: 3,
    EnteringCode: 4,
    CheckingCodeLoader: 5,
    Success: 6,
    WrongCode: 7,
    AlreadyExists: 8,
    SentForgotEmail: 9,
    FatalError: 10,
    CodeRateLimit: 11,
    TriesOut: 12,
    ForgotRateLimit: 13
  };

  let autofocusInputs: Array<HTMLInputElement|null> = $state([]);
  let currentState: number = $state(StateEnum.EnteringEmail);
  let createdId: string = $state("");

  let emailConfirmationToken = "";
  let timerSeconds: number = $state(0);
  let timerDecreaseInterval: number|undefined = undefined;

  $effect(() => {
    const input: HTMLInputElement|null = autofocusInputs[step];
    if (input) {
      input.focus();
    }
  });

  // Buttons event listeners
  const stepByState = () => {
    const steps: Record<number, number> = {};

    steps[StateEnum.EnteringEmail] = 0;
    steps[StateEnum.EnteringName] = 1;
    steps[StateEnum.EnteringTitle] = 2;
    steps[StateEnum.SendingEmailLoader] = 3;
    steps[StateEnum.EnteringCode] = 3;
    steps[StateEnum.CheckingCodeLoader] = 3;
    steps[StateEnum.Success] = -1;
    steps[StateEnum.WrongCode] = -1;
    steps[StateEnum.AlreadyExists] = -1;
    steps[StateEnum.SentForgotEmail] = -1;
    steps[StateEnum.FatalError] = -1;
    steps[StateEnum.CodeRateLimit] = 3;
    steps[StateEnum.TriesOut] = 3;
    steps[StateEnum.ForgotRateLimit] = -1;

    return steps[currentState];
  };

  const back = async () => {
    const states: Record<number, number> = {};

    states[StateEnum.EnteringName] = StateEnum.EnteringEmail;
    states[StateEnum.EnteringTitle] = StateEnum.EnteringName;
    states[StateEnum.EnteringCode] = StateEnum.EnteringTitle;
    states[StateEnum.WrongCode] = StateEnum.EnteringCode;
    
    currentState = states[currentState];

    step = stepByState();
  };

  const submitEmail = async () => {
    currentState = StateEnum.EnteringName;
    step = stepByState();
  };

  const submitName = async () => {
    currentState = StateEnum.EnteringTitle;
    step = stepByState();
  };

  const submitTitle = async () => {
    currentState = StateEnum.SendingEmailLoader;
    step = stepByState();

    await sendCodeCertCreation(email, {
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
              currentState = StateEnum.EnteringTitle;
              step = stepByState();
              clearInterval(timerDecreaseInterval);
            }
          }, 1000);
        };

        if (
          codeError === ERROR_IP_RATE_LIMIT || 
          codeError === ERROR_EMAIL_RATE_LIMIT
        ) {
          currentState = StateEnum.CodeRateLimit;
          const currentTimestamp = Math.ceil(Date.now() / 1000);
          let rateTimestamp = data["timestamp"];
          timerSeconds = rateTimestamp - currentTimestamp;
          setRateLimitInterval(rateTimestamp);
        } else if (codeError === ERROR_ALREADY_EXISTS) {
          currentState = StateEnum.AlreadyExists;
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
      },
    });
  };

  const submitCode = async () => {
    currentState = StateEnum.CheckingCodeLoader;

    await createCert({
      email: email,
      name: name,
      title: title,
      code: code,
      token: emailConfirmationToken
    }, {
      onSuccess: (data) => {
        currentState = StateEnum.Success;
        step = stepByState();
        createdId = data.id + "";
      },
      onError: (codeError, message, data) => {
        console.error(codeError, message);

        if (codeError === ERROR_INVALID_CODE) {
          currentState = StateEnum.WrongCode;
        } else if (codeError === ERROR_ALREADY_EXISTS) {
          currentState = StateEnum.AlreadyExists;
        } else if (codeError === ERROR_TRIES_OUT) {
          currentState = StateEnum.TriesOut;

          clearInterval(timerDecreaseInterval);

          const currentTimestamp = Math.ceil(Date.now() / 1000);
          const timerTimestamp = data["timestamp"];
          timerSeconds = timerTimestamp - currentTimestamp;

          timerDecreaseInterval = setInterval(() => {
            const currentTimestamp = Math.ceil(Date.now() / 1000);
            timerSeconds = timerTimestamp - currentTimestamp;

            if (timerSeconds < 0) {
              currentState = StateEnum.EnteringTitle;
              step = stepByState();
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
      }
    });
  };

  const goToForgotCert = async () => {
    currentState = StateEnum.SendingEmailLoader;
    step = -1;

    await forgotCert(email, {
      onSuccess: (data) => {
        currentState = StateEnum.SentForgotEmail;
      },
      onError: (codeError, message, data) => {
        console.error(codeError, message);


        const setRateLimitInterval = (rateTimestamp: number) => {
          clearInterval(timerDecreaseInterval);

          timerDecreaseInterval = setInterval(() => {
            const currentTimestamp = Math.ceil(Date.now() / 1000);
            timerSeconds = rateTimestamp - currentTimestamp;

            if (timerSeconds < 0) {
              currentState = StateEnum.AlreadyExists;
              step = stepByState();
              clearInterval(timerDecreaseInterval);
            }
          }, 1000);
        };

        if (
          codeError === ERROR_EMAIL_RATE_LIMIT ||
          codeError === ERROR_IP_RATE_LIMIT
        ) {
          currentState = StateEnum.ForgotRateLimit;
          const currentTimestamp = Math.ceil(Date.now() / 1000);
          let rateTimestamp = data["timestamp"];
          timerSeconds = rateTimestamp - currentTimestamp;
          setRateLimitInterval(rateTimestamp);
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

  onMount(() => {
    return () => {
      clearInterval(timerDecreaseInterval);
    };
  });
</script>

<svelte:head>
  <title>Отримати сертифікат — Асоціація пупсіків України</title>
</svelte:head>

<main class="sm:px-20 px-3 pb-12 w-full bg-brand-primary" transition:slide>
  {#if step >= 0}
  <h1 class="mb-2 font-unbounded text-white">Щоб стати пупсіком залишилося {steps-step} кроків!</h1>
  <div class="mb-5 flex flex-row gap-3 w-full items-center">
    {#each [...new Array(steps).keys()] as stepBar}
      {#if stepBar < step}
        <div class="flex-1 rounded-full bg-white h-1"></div>
      {:else}
        <div class="flex-1 rounded-full bg-orange-300 h-1"></div>
      {/if}
    {/each}
  </div>
  {/if}

  {#if currentState === StateEnum.EnteringEmail}
    <h2 class="mb-1 font-bold text-white">Введіть пошту</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Пошта потрібна, щоб ідентифікувати власника сертифіката. Завдяки власній пошті Ви зможете контролювати свій сертифікат у базі.</p>
    <form onsubmit={e => {e.preventDefault();submitEmail();}}>
      <input class="input mb-2" type="email" placeholder="me@example.com" required bind:this={autofocusInputs[0]} bind:value={email}>
      <button class="button" type="submit">Далі</button>
    </form>
  {:else if currentState === StateEnum.EnteringName}
    <h2 class="mb-1 font-bold text-white">Введіть ПІБ</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">ПІБ потрібен, щоб відобразити на сертифікаті.</p>
    <form onsubmit={e => {e.preventDefault();submitName();}}>
      <input class="input mb-2" type="text" placeholder="Григорій Мельник Сергійович" required minlength="2" maxlength="200" bind:this={autofocusInputs[1]} bind:value={name}>
      <div class="flex flex-row">
        <button class="button-primary text-gray-200" type="button" onclick={ back }>Назад</button>
        <button class="button" type="submit">Далі</button>
      </div>
    </form>
  {:else if currentState === StateEnum.EnteringTitle}
    <h2 class="mb-1 font-bold text-white">Придумайте собі титул</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Титул потрібен, щоб відобразити на сертифікаті.</p>
    <form onsubmit={e => {e.preventDefault();submitTitle();}}>
      <input class="input mb-2" type="text" placeholder="Король пупсіків" required minlength="1" maxlength="200" bind:this={autofocusInputs[2]} bind:value={title}>
      <div class="flex flex-row">
        <button class="button-primary text-gray-200" type="button" onclick={ back }>Назад</button>
        <button class="button" type="submit">Далі</button>
      </div>
    </form>
  {:else if currentState === StateEnum.SendingEmailLoader || currentState === StateEnum.CheckingCodeLoader}
    <Loader />
  {:else if currentState === StateEnum.EnteringCode}
    <h2 class="mb-1 font-bold text-white">Введіть код з пошти</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Код потрібен, щоб підтвердити належність пошти.</p>
    <form onsubmit={e => {e.preventDefault();submitCode();}}>
      <input class="input mb-2 uppercase" type="text" placeholder="AAA111BBB" required minlength="9" maxlength="9" pattern={EMAIL_CODE_PATTERN} bind:this={autofocusInputs[3]} bind:value={code}>
      <div class="flex flex-row">
        <button class="button-primary text-gray-200" type="button" onclick={ back }>Назад</button>
        <button class="button" type="submit">Отримати сертифікат</button>
      </div>
    </form>
  {:else if currentState === StateEnum.Success}
    <h2 class="mb-1 font-bold text-white">Успіх!</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Сертифікат було створено!</p>
    <a class="button w-fit" href={"/cert/" + createdId}>Поглянути</a>
  {:else if currentState === StateEnum.WrongCode}
    <h2 class="mb-1 font-bold text-white">Хибний код!</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Будьте обачні, у Вас обмежена кількість спроб.</p>
    <button class="button-primary text-gray-200" type="button" onclick={ back }>Назад</button>
  {:else if currentState === StateEnum.AlreadyExists}
    <h2 class="mb-1 font-bold text-white">Вже зайнято!</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">На вказану пошту вже прив'язаний сертифікат.</p>
    <button class="button" onclick={ goToForgotCert }>Відправити сертифікат на пошту</button>
  {:else if currentState === StateEnum.SentForgotEmail}
    <h2 class="mb-1 font-bold text-white">Успіх!</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Повідомлення було відправлено!</p>
    <a class="button w-fit" href="/">Повернутися</a>
  {:else if currentState === StateEnum.FatalError}
    <h2 class="mb-1 font-bold text-white">Невідома помилка</h2>
    <p class="mb-2 block max-w-[500px] text-white text-xs italic">Перезапустіть сторінку і повторіть спробу пізніше.</p>
  {:else if currentState === StateEnum.CodeRateLimit}
    <h2 class="mb-1 font-bold text-white">Почекайте, щоб відправити код знову!</h2>
    <p class="mb-2 text-white">{ durationFormat(timerSeconds) }</p>
  {:else if currentState === StateEnum.TriesOut}
    <h2 class="mb-1 font-bold text-white">Спроби закінчилися!</h2>
    <p class="mb-2 text-white">{ durationFormat(timerSeconds) }</p>
  {:else if currentState === StateEnum.ForgotRateLimit}
    <h2 class="mb-1 font-bold text-white">Почекайте, щоб відправити сертифікат знову!</h2>
    <p class="mb-2 text-white">{ durationFormat(timerSeconds) }</p>
  {/if}
</main>
