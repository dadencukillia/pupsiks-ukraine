<script lang="ts">
  import voidPantograph from "$lib/assets/void.jpg";

  const {
    certName,
    certTitle,
    certId
  }: {
    certName: string,
    certTitle: string,
    certId: string
  } = $props();
</script>

<div
  style={`background-image:url(${JSON.stringify(voidPantograph)});`}
  class="card flex flex-col md:flex-1 p-5 rounded-xl bg-center max-w-[600px] aspect-2/1 overflow-hidden"
>
  <div class="text-shadow-lg text-shadow-white truncate">
    <h1>Сертифікований пупсік</h1>
    <h1 class="font-unbounded font-bold overflow-hidden text-ellipsis">{ certName }</h1>
    <h2 class="overflow-hidden text-ellipsis">{ certTitle }</h2>
  </div>
  <div class="flex w-full flex-1 flex-col justify-end items-start text-xs overflow-hidden text-ellipsis">
    <b>Сертифіковано АПУ №{ certId }</b>
  </div>
</div>
