<script setup lang="ts">
import { type Post } from '@/types/post'
import TagList from './TagList.vue';
const props = defineProps({
  // wtf is this shit
  // is this how you pass an object as props?
  // ts was a mistake
  post: { type: Object as () => Post }
});

</script>

<template>
  <div v-if="props.post" style="
    width: 100%; 
    display: flex;
    padding-bottom: 40px;
    flex-direction: column;
  ">
    <h3 style="margin-bottom: 5px;">
      <RouterLink to="">{{ props.post.title }}</RouterLink>
    </h3>
    <div style="margin-bottom: 5px;">
      author: <b>{{ props.post.user.name }}</b> <br/>
      <i>rating : {{ props.post.rating }}/5</i>
      <i style="float: right;">{{ new Date(props.post.createdAt).toLocaleDateString(
        'en-GB', {
        day: 'numeric', month: 'numeric', year: 'numeric'
      }) }}</i>
    </div>
    <img v-if="props.post.mainImageUrl" v-bind:src="props.post.mainImageUrl" />
    <img v-else src="@/assets/missing_image.webp" />
    <div style="flex:50%">
      <p>
        {{ props.post.shortText }}
      </p>
      likes: {{ props.post.likes }}
     <TagList :tags="props.post.tags" /> 
    </div>
  </div>
</template>

<style scoped>
img {
  max-width: 256px;
  margin-bottom: 15px;
}
</style>
