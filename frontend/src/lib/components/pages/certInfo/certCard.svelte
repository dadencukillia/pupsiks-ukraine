<script lang="ts">
  import voidPantograph from "$lib/assets/void.jpg?enhanced&w=600;450;300;150;75";

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
  class="card relative flex flex-col md:flex-1 p-5 max-w-[600px] aspect-2/1 overflow-hidden rounded-xl"
>
  <enhanced:img src={ voidPantograph } alt="void pantograph" class="absolute left-0 top-0 w-full h-full object-center -z-1" sizes="(min-width:883px) 600px, (min-width:768px) 450px, (min-width: 640px) calc(100vw - 10rem), calc(100vw - 1.5rem)" />
  <div class="text-shadow-lg text-shadow-white truncate">
    <h1>Сертифікований пупсік</h1>
    <h1 class="font-unbounded font-bold overflow-hidden text-ellipsis">{ certName }</h1>
    <h2 class="overflow-hidden text-ellipsis">{ certTitle }</h2>
  </div>
  <div class="flex w-full flex-1 flex-col justify-end items-start text-xs overflow-hidden text-ellipsis">
    <b>Сертифіковано АПУ №{ certId }</b>
  </div>
</div>
