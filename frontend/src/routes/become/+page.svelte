<script lang="ts">
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
  import { Timer } from "$lib/utils/reactiveTimer.svelte";
  import { onDestroy} from "svelte";
  import { FiniteStateMachine, type StatesRouter } from "svelte-state-machine";
  import { slide } from "svelte/transition";

  // Timer
  const timer = new Timer();

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
      [FSM.enum.WrongCode,          () => 3],
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
  const maxSteps: number = 4;
  let step: number = $derived(stepByState());

  let email: string = $state("");
  let name: string = $state("");
  let title: string = $state("");
  let code: string = $state("");

  let autofocusInputs: Array<HTMLInputElement|null> = $state([]);

  let createdId: string = $state("");
  let emailConfirmationToken: string = "";

  $effect(() => {
    const input: HTMLInputElement|null = autofocusInputs[step];
    if (input) {
      input.focus();
    }
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
      onError: (matcher, _message, data) => {
        const onRateLimit = () => {
          FSM.state = FSM.enum.ForgotRateLimit;

          timer.onEnd = _ => {
            FSM.state = FSM.enum.EnteringTitle;
          };

          timer.runTimestampSeconds(data["timestamp"] as number);
        };

        matcher.match({
          EMAIL_RATE_LIMIT: onRateLimit,
          IP_RATE_LIMIT: onRateLimit,
          ALREADY_EXISTS: () => { FSM.state = FSM.enum.AlreadyExists },
          default: () => { FSM.state = FSM.enum.FatalError }
        });
      }
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
      onError: (matcher, _message, data) => {
        matcher.match({
          INVALID_CODE: () => { FSM.state = FSM.enum.WrongCode },
          ALREADY_EXISTS: () => { FSM.state = FSM.enum.AlreadyExists },
          TRIES_OUT: () => { 
            FSM.state = FSM.enum.TriesOut;

            timer.onEnd = _ => {
              FSM.state = FSM.enum.EnteringTitle;
            }

            timer.runTimestampSeconds(data["timestamp"] as number);
          },
          default: () => { FSM.state = FSM.enum.FatalError }
        });
      }
    });
  };

  const goToForgotCert = async () => {
    FSM.state = FSM.enum.SendingEmailLoader;

    await forgotCert(email, {
      onSuccess: (_data) => {
        FSM.state = FSM.enum.SentForgotEmail;
      },
      onError: (matcher, _message, data) => {
        const onRateLimit = () => {
          FSM.state = FSM.enum.ForgotRateLimit;

          timer.onEnd = _ => {
            FSM.state = FSM.enum.AlreadyExists;
          };

          timer.runTimestampSeconds(data["timestamp"] as number);
        };

        matcher.match({
          EMAIL_RATE_LIMIT: onRateLimit,
          IP_RATE_LIMIT: onRateLimit,
          default: () => {
            FSM.state = FSM.enum.FatalError;
          }
        });
      },
    });
  };

  onDestroy(() => {
    timer.stop(false);
  })
</script>

<svelte:head>
  <title>Отримати сертифікат — Асоціація пупсіків України</title>
</svelte:head>

<main class="sm:px-20 px-3 pb-12 w-full bg-brand-primary" transition:slide>
  <Progressbar step={ step } maxSteps={ maxSteps } />

  {#if FSM.check.EnteringEmail()}
    <EnteringEmailState goNext={ submitEmail } bind:input={ autofocusInputs[0] } bind:emailValue={ email } />
  {:else if FSM.check.EnteringName()}
    <EnteringNameState goNext={ submitName } goBack={ back } bind:input={ autofocusInputs[1] } bind:nameValue={ name } />
  {:else if FSM.check.EnteringTitle()}
    <EnteringTitleState goNext={ submitTitle } goBack={ back } bind:input={ autofocusInputs[2] } bind:titleValue={ title } />
  {:else if FSM.check.SendingEmailLoader() || FSM.check.CheckingCodeLoader()}
    <Loader />
  {:else if FSM.check.EnteringCode()}
    <EnteringCodeState goNext={ submitCode } goBack={ back } bind:input={ autofocusInputs[3] } bind:codeValue={ code } />
  {:else if FSM.check.Success()}
    <SuccessCreateState createdId={ createdId } />
  {:else if FSM.check.WrongCode()}
    <WrongCodeState back={ back } />
  {:else if FSM.check.AlreadyExists()}
    <AlreadyExistsState goToForgotCert={ goToForgotCert } />
  {:else if FSM.check.SentForgotEmail()}
    <SuccessForgotState />
  {:else if FSM.check.FatalError()}
    <FatalErrorState errorTitle="Невідома помилка!" />
  {:else if FSM.check.CodeRateLimit()}
    <CodeRateLimitState timerSeconds={ timer.remainSeconds } />
  {:else if FSM.check.TriesOut()}
    <TriesOutState timerSeconds={ timer.remainSeconds } />
  {:else if FSM.check.ForgotRateLimit()}
    <ForgotRateLimitState timerSeconds={ timer.remainSeconds } />
  {/if}
</main>
