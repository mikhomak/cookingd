<script setup lang="ts">
import { type Post } from '@/types/post'
import TagList from './TagList.vue';
import PostTopDetails from '@/components/posts/PostTopDetails.vue'
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
      <router-link :to="`/post/${props.post.id}`">{{ props.post.title }}</router-link>
    </h3>
    
    <!-- I fucking hate ts so fucking much -->
    <!-- @vue-ignore -->
    <PostTopDetails :name="props!.post!.user!.name" :rating="props!.post!.rating!" :createdAt="props.post.createdAt"/>
    <img v-if="props.post.mainImageUrl" v-bind:src="props.post.mainImageUrl" class="post_image"/>
    <img v-else src="@/assets/missing_image.webp" class="post_image"/>
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

</style>
