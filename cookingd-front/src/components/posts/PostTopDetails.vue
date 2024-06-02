<script setup lang="ts">
const props = defineProps({
    name: String,
    rating: Number,
    createdAt: Date
});
</script>
<template>

<div style="margin-bottom: 5px;">
      author: <b>{{ props!.name }}</b> <br />
      <i>rating : {{ props!.rating }}/5</i>
      <i style="float: right;">{{ new Date(props!.createdAt!).toLocaleDateString(
    'en-GB', {
    day: 'numeric', month: 'numeric', year: 'numeric'
  }) }}</i>
    </div>
</template>