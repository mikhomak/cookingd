<script setup lang="ts">
const props= defineProps({
  title: String,
  main_image_url: String,
  rating: Number,
  likes: Number,
  date: date
});
</script>

<template>
  {{ props.title }}
  {{ props.likes }}
</template>

<style scoped>

</style>
