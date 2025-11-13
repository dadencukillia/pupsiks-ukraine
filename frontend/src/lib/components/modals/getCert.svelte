<script lang="ts">
  import { goto } from "$app/navigation";
    import { CERT_ID_PATTERN } from "$lib/api/regexPatterns";
    import { slide } from "svelte/transition";

  let code = $state("");
  let hide = $state(false);

  const submitCertCode = () => {
    hide = true;
    goto("/cert/" + code);
  };
</script>

{#if !hide}
<div class="w-full bg-brand-primary p-3" transition:slide>
  <div class="flex flex-col gap-2">
    <h1 class="font-unbounded text-white">Введіть серійний номер сертифікату</h1>
    <form class="flex flex-row gap-2" onsubmit={e => {e.preventDefault();submitCertCode();}}>
      <input class="input" type="text" placeholder="Серійний номер" minlength="22" maxlength="22" required pattern={CERT_ID_PATTERN} bind:value={code}>
      <button class="button-primary" type="submit">Перевірити сертифікат</button>
    </form>
  </div>
</div>
{/if}
