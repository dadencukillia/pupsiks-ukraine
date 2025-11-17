<script lang="ts">
  import FeatureCard from "./featureCard.svelte";

  const {
    usersCount
  }: {
    usersCount: string
  } = $props();
</script>

<section>
  <h1 class="font-unbounded mb-3">Чому слід вибрати нас?</h1>
  <p class="mb-2">АПУ єдина в своєму роді і має багато ключових переваг. Ось деякі з них:</p>
  <div class="w-full flex flex-col sm:flex-row gap-2">
    <FeatureCard title="⏳ Швидко" text="Отримуйте сертифікат менше ніж за 5 хвилин." />
    <FeatureCard title="💽 Велика база" text={`Наша база вже має ${usersCount} зареєстрованих пупсіків!`} />
    <FeatureCard title="🔒 Надійно" text="Дані з бази зникнуть лише якщо станеться щось погане!" />
  </div>
</section>
