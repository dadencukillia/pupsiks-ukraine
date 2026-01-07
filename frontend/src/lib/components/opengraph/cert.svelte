<svelte:options css="injected" />

<script lang="ts">
  import logo from '$lib/assets/logo.png?inline';
  import stamp from '$lib/assets/stamp.png?inline';

  let {
    id,
    name,
    title
  }: {
    id: string,
    name: string,
    title: string
  } = $props();

  const stampX = id.substring(0, 5).split("").reduce((acc, v) => acc * v.charCodeAt(0) * 27, 1) % (1200 - 256);
  const stampY = id.substring(6, 11).split("").reduce((acc, v) => acc * v.charCodeAt(0) * 17, 1) % (630 - 256 - 200) + 200;
  const rotateStamp = id.substring(12).split("").reduce((acc, v) => acc * v.charCodeAt(0) * 7, 1) % (359);
</script>

<main>
  <div class="container">
    <img class="logo" src={ logo } alt="APU Logo">
    <h1>{ name }</h1>
    <p>{ title }</p>
  </div>
  <div class="brand-line">Сертифіковано APU №{ id }</div>
  <img class="stamp" src={ stamp } alt="Stamp" style={`top:${stampY};left:${stampX};transform:rotate(${rotateStamp}deg);`}>
</main>

<style>
  * {
    --color-brand-primary: #fd4a04;
    --font-unbounded: Unbounded, sans-serif;
    --font-opensans: "Open Sans", sans-serif;
  }

  main {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
  }

  .container {
    display: flex;
    flex-direction: column;
    flex: 1;
    width: 100%;
    height: 100%;
    background-color: white;
    color: black;
    padding: 10px;
  }

  .brand-line {
    display: flex;
    flex-direction: column;
    background-color: #fd4a04;
    color: #fd4000;
    font-size: 24px;
  }

  .logo {
    height: 64px;
  }

  .stamp {
    position: absolute;
    width: 256px;
  }

  h1, p {
    width: 100%;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 3;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  h1 {
    color: #fd4a04;
    font-family: Unbounded, sans-serif;
    font-weight: bold;
    font-size: 48px;
    margin-bottom: 0;
  }

  p {
    margin: 0;
    font-size: 32px;
    -webkit-line-clamp: 5;
  }
</style>
