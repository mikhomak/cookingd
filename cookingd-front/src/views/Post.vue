<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import { useRoute } from 'vue-router';
import PostTopDetails from '@/components/posts/PostTopDetails.vue'
import TagList from '@/components/posts/TagList.vue';

const route = useRoute();

const POST_QUERY = gql`
query($post_id: String!){
  postForId(postId : $post_id){
    title
    text
    likes
    rating
    createdAt
    allowComments
    allowLikes
    mainImageUrl
    tags{
      name
    }
    user{
      name
    }
  }
}
`

const { result, loading, error } = useQuery(POST_QUERY, () => ({
  post_id: route.params.post_id
}));

</script>

<template>
  <div v-if="!loading">
    <h3>{{ result.postForId.title }}</h3>
    <PostTopDetails :name="result.postForId.user.name" :rating="result.postForId.rating"
      :createdAt="result.postForId.createdAt" />
    <img v-if="result.postForId.mainImageUrl" v-bind:src="result.postForId.mainImageUrl" class="post_image" />
    <img v-else src="@/assets/missing_image.webp" class="post_image"/>
    <div style="flex:50%">
      <p>
        {{ result.postForId.text }}
      </p>
      likes: {{ result.postForId.likes }}
      <TagList :tags="result.postForId.tags" />
    </div>

  </div>

  <div v-else-if="loading">
    <h3 style="color: greenyellow">Loading posts...</h3>
  </div>
</template>