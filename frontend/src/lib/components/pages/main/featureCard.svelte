<script lang="ts">
  const props: {
    title: string,
    text: string
  } = $props();
</script>

<div class="flex-1 bg-brand-primary rounded-xl p-3 text-white">
  <h1 class="font-bold">{props.title}</h1>
  <p>{props.text}</p>
</div>
