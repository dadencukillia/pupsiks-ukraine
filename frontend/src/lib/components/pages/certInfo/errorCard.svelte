<script lang="ts">
  const {
    errorText
  }: {
    errorText: string
  } = $props();
</script>

<div class="w-full py-12 flex items-center justify-center text-center flex-col gap-3 text-white bg-red-500 rounded-xl">
  <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-exclamation-circle-fill" viewBox="0 0 16 16">
    <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0M8 4a.905.905 0 0 0-.9.995l.35 3.507a.552.552 0 0 0 1.1 0l.35-3.507A.905.905 0 0 0 8 4m.002 6a1 1 0 1 0 0 2 1 1 0 0 0 0-2"/>
  </svg>
  <h1 class="font-bold">{ errorText }</h1>
</div>
