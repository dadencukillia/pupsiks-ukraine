<script lang="ts">
  import { durationFormat } from "$lib/utils/timeFormat";

  const {
    timerSeconds
  }: {
    timerSeconds: number
  } = $props();
</script>

<h2 class="mb-1 font-bold text-white">Почекайте, щоб відправити сертифікат знову!</h2>
<p class="mb-2 text-white">{ durationFormat(timerSeconds) }</p>
