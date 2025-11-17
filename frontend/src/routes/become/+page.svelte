<script lang="ts">
  import { ERROR_ALREADY_EXISTS, ERROR_EMAIL_RATE_LIMIT, ERROR_INVALID_CODE, ERROR_IP_RATE_LIMIT, ERROR_TRIES_OUT } from "$lib/api/configs";
  import { createCert, forgotCert } from "$lib/api/requests/cert_crud";
  import { sendCodeCertCreation } from "$lib/api/requests/code_confirmation";
  import Loader from "$lib/components/loader.svelte";
  import Progressbar from "$lib/components/pages/become/progressbar.svelte";
  import AlreadyExistsState from "$lib/components/pages/become/states/alreadyExistsState.svelte";
  import CodeRateLimitState from "$lib/components/pages/become/states/codeRateLimitState.svelte";
  import EnteringCodeState from "$lib/components/pages/become/states/enteringCodeState.svelte";
  import EnteringEmailState from "$lib/components/pages/become/states/enteringEmailState.svelte";
  import EnteringNameState from "$lib/components/pages/become/states/enteringNameState.svelte";
  import EnteringTitleState from "$lib/components/pages/become/states/enteringTitleState.svelte";
  import FatalErrorState from "$lib/components/pages/become/states/fatalErrorState.svelte";
  import ForgotRateLimitState from "$lib/components/pages/become/states/forgotRateLimitState.svelte";
  import SuccessCreateState from "$lib/components/pages/become/states/successCreateState.svelte";
  import SuccessForgotState from "$lib/components/pages/become/states/successForgotState.svelte";
  import TriesOutState from "$lib/components/pages/become/states/triesOutState.svelte";
  import WrongCodeState from "$lib/components/pages/become/states/wrongCodeState.svelte";
  import { FiniteStateMachine, type StatesRouter } from "$lib/utils/finiteStateMachine.svelte";
  import { onMount } from "svelte";
  import { slide } from "svelte/transition";

  // FSM
  const FSM = new FiniteStateMachine(
    "EnteringEmail",
    "EnteringName",
    "EnteringTitle",
    "SendingEmailLoader",
    "EnteringCode",
    "CheckingCodeLoader",
    "Success",
    "WrongCode",
    "AlreadyExists",
    "SentForgotEmail",
    "FatalError",
    "CodeRateLimit",
    "TriesOut",
    "ForgotRateLimit",
  );

  const backSectionsRouter: StatesRouter = {
    stateFor(state): number {
      switch (state) {
        case FSM.enum.EnteringName: return FSM.enum.EnteringEmail;
        case FSM.enum.EnteringTitle: return FSM.enum.EnteringName;
        case FSM.enum.EnteringCode: return FSM.enum.EnteringTitle;
        case FSM.enum.WrongCode: return FSM.enum.EnteringCode;
        default: return FSM.enum.EnteringEmail;
      };
    }
  };

  const stepByState = () => {
    return FSM.match(
      [FSM.enum.EnteringEmail,      () => 0],
      [FSM.enum.EnteringName,       () => 1],
      [FSM.enum.EnteringTitle,      () => 2],
      [FSM.enum.SendingEmailLoader, () => 3],
      [FSM.enum.EnteringCode,       () => 3],
      [FSM.enum.CheckingCodeLoader, () => 3],
      [FSM.enum.Success,            () => -1],
      [FSM.enum.WrongCode,          () => -1],
      [FSM.enum.AlreadyExists,      () => -1],
      [FSM.enum.SentForgotEmail,    () => -1],
      [FSM.enum.FatalError,         () => -1],
      [FSM.enum.CodeRateLimit,      () => 3],
      [FSM.enum.TriesOut,           () => 3],
      [FSM.enum.ForgotRateLimit,    () => -1],
      [null,                        () => step]
    )!;
  };

  // States
  const steps = 4;
  let step = $state<number>(0);

  let email: string = $state<string>("");
  let name: string = $state<string>("");
  let title: string = $state<string>("");
  let code = $state<string>("");

  let autofocusInputs: Array<HTMLInputElement|null> = $state([]);

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

  FSM.subscribeStateChanges((_fsm, _prevState, _newState) => {
    step = stepByState();
  });

  // Buttons event handlers
  const back = async () => {
    FSM.next(backSectionsRouter);
  };

  const submitEmail = async () => {
    FSM.state = FSM.enum.EnteringName;
  };

  const submitName = async () => {
    FSM.state = FSM.enum.EnteringTitle;
  };

  const submitTitle = async () => {
    FSM.state = FSM.enum.SendingEmailLoader;

    await sendCodeCertCreation(email, {
      onSuccess: (data) => {
        emailConfirmationToken = data.token;
        FSM.state = FSM.enum.EnteringCode;
      },
      onError: (codeError, message, data) => {
        console.error(codeError, message);

        const setRateLimitInterval = (rateTimestamp: number) => {
          clearInterval(timerDecreaseInterval);

          timerDecreaseInterval = setInterval(() => {
            const currentTimestamp = Math.ceil(Date.now() / 1000);
            timerSeconds = rateTimestamp - currentTimestamp;

            if (timerSeconds < 0) {
              clearInterval(timerDecreaseInterval);
              FSM.state = FSM.enum.EnteringTitle;
            }
          }, 1000);
        };

        if (
          codeError === ERROR_IP_RATE_LIMIT || 
          codeError === ERROR_EMAIL_RATE_LIMIT
        ) {
          FSM.state = FSM.enum.CodeRateLimit;
          const currentTimestamp = Math.ceil(Date.now() / 1000);
          let rateTimestamp = data["timestamp"];
          timerSeconds = rateTimestamp - currentTimestamp;
          setRateLimitInterval(rateTimestamp);
        } else if (codeError === ERROR_ALREADY_EXISTS) {
          FSM.state = FSM.enum.AlreadyExists;
        } else {
          FSM.state = FSM.enum.FatalError;
        }
      },
      onFatal: (error) => {
        console.error(error);
        FSM.state = FSM.enum.FatalError;
      },
    });
  };

  const submitCode = async () => {
    FSM.state = FSM.enum.CheckingCodeLoader;

    await createCert({
      email: email,
      name: name,
      title: title,
      code: code,
      token: emailConfirmationToken
    }, {
      onSuccess: (data) => {
        FSM.state = FSM.enum.Success;
        createdId = data.id + "";
      },
      onError: (codeError, message, data) => {
        console.error(codeError, message);

        if (codeError === ERROR_INVALID_CODE) {
          FSM.state = FSM.enum.WrongCode;
        } else if (codeError === ERROR_ALREADY_EXISTS) {
          FSM.state = FSM.enum.AlreadyExists;
        } else if (codeError === ERROR_TRIES_OUT) {
          FSM.state = FSM.enum.TriesOut;

          clearInterval(timerDecreaseInterval);

          const currentTimestamp = Math.ceil(Date.now() / 1000);
          const timerTimestamp = data["timestamp"];
          timerSeconds = timerTimestamp - currentTimestamp;

          timerDecreaseInterval = setInterval(() => {
            const currentTimestamp = Math.ceil(Date.now() / 1000);
            timerSeconds = timerTimestamp - currentTimestamp;

            if (timerSeconds < 0) {
              FSM.state = FSM.enum.EnteringTitle;
              clearInterval(timerDecreaseInterval);
            }
          }, 1000);
        } else {
          FSM.state = FSM.enum.FatalError;
        }
      },
      onFatal: (error) => {
        console.error(error);
        FSM.state = FSM.enum.FatalError;
      }
    });
  };

  const goToForgotCert = async () => {
    FSM.state = FSM.enum.SendingEmailLoader;
    step = -1;

    await forgotCert(email, {
      onSuccess: (_data) => {
        FSM.state = FSM.enum.SentForgotEmail;
      },
      onError: (codeError, message, data) => {
        console.error(codeError, message);


        const setRateLimitInterval = (rateTimestamp: number) => {
          clearInterval(timerDecreaseInterval);

          timerDecreaseInterval = setInterval(() => {
            const currentTimestamp = Math.ceil(Date.now() / 1000);
            timerSeconds = rateTimestamp - currentTimestamp;

            if (timerSeconds < 0) {
              FSM.state = FSM.enum.AlreadyExists;
              clearInterval(timerDecreaseInterval);
            }
          }, 1000);
        };

        if (
          codeError === ERROR_EMAIL_RATE_LIMIT ||
          codeError === ERROR_IP_RATE_LIMIT
        ) {
          FSM.state = FSM.enum.ForgotRateLimit;
          const currentTimestamp = Math.ceil(Date.now() / 1000);
          let rateTimestamp = data["timestamp"];
          timerSeconds = rateTimestamp - currentTimestamp;
          setRateLimitInterval(rateTimestamp);
        } else {
          FSM.state = FSM.enum.FatalError;
        }
      },
      onFatal: (error) => {
        console.error(error);
        FSM.state = FSM.enum.FatalError;
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
  <Progressbar step={ step } maxSteps={ steps } />

  {#if FSM.state === FSM.enum.EnteringEmail}
    <EnteringEmailState goNext={ submitEmail } bind:input={ autofocusInputs[0] } bind:emailValue={ email } />
  {:else if FSM.state === FSM.enum.EnteringName}
    <EnteringNameState goNext={ submitName } goBack={ back } bind:input={ autofocusInputs[1] } bind:nameValue={ name } />
  {:else if FSM.state === FSM.enum.EnteringTitle}
    <EnteringTitleState goNext={ submitTitle } goBack={ back } bind:input={ autofocusInputs[2] } bind:titleValue={ title } />
  {:else if FSM.state === FSM.enum.SendingEmailLoader || FSM.state === FSM.enum.CheckingCodeLoader}
    <Loader />
  {:else if FSM.state === FSM.enum.EnteringCode}
    <EnteringCodeState goNext={ submitCode } goBack={ back } bind:input={ autofocusInputs[3] } bind:codeValue={ code } />
  {:else if FSM.state === FSM.enum.Success}
    <SuccessCreateState createdId={ createdId } />
  {:else if FSM.state === FSM.enum.WrongCode}
    <WrongCodeState back={ back } />
  {:else if FSM.state === FSM.enum.AlreadyExists}
    <AlreadyExistsState goToForgotCert={ goToForgotCert } />
  {:else if FSM.state === FSM.enum.SentForgotEmail}
    <SuccessForgotState />
  {:else if FSM.state === FSM.enum.FatalError}
    <FatalErrorState errorTitle="Невідома помилка!" />
  {:else if FSM.state === FSM.enum.CodeRateLimit}
    <CodeRateLimitState timerSeconds={ timerSeconds } />
  {:else if FSM.state === FSM.enum.TriesOut}
    <TriesOutState timerSeconds={ timerSeconds } />
  {:else if FSM.state === FSM.enum.ForgotRateLimit}
    <ForgotRateLimitState timerSeconds={ timerSeconds } />
  {/if}
</main>
