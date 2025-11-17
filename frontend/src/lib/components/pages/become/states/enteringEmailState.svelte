<script lang="ts">
  let {
    goNext,
    input = $bindable(),
    emailValue = $bindable()
  }: {
    goNext: () => void,
    input: HTMLInputElement|null,
    emailValue: string
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">Введіть пошту</h2>
<p class="mb-2 block max-w-[500px] text-white text-xs italic">Пошта потрібна, щоб ідентифікувати власника сертифіката. Завдяки власній пошті Ви зможете контролювати свій сертифікат у базі.</p>
<form onsubmit={ e => { e.preventDefault(); goNext(); } }>
  <input class="input mb-2" type="email" placeholder="me@example.com" required bind:this={ input } bind:value={ emailValue }>
  <button class="button" type="submit">Далі</button>
</form>
