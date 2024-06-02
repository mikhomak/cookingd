<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import ShortPost from '@/components/posts/ShortPost.vue'
import Pagination from '@/components/Pagination.vue'
import { ref } from 'vue';
import router from '@/router';
import { useRoute } from 'vue-router';

const currentPage = ref(0);
const route = useRoute();
if (route.params.page) {
  //@ts-ignore
  currentPage.value = parseInt(route.params.page)
}

const LATEST_POSTS_QUERY = gql`
  query($page: Int!){
  latestPosts(page:$page){
  posts{
    id
    title
    likes
    rating
    text
    shortText
    createdAt
    mainImageUrl
    tags{
      name
    }
    user{
      name
    }
  }
  pages
  }
}
`

const { result, loading, error } = useQuery(LATEST_POSTS_QUERY, () => ({
  page: currentPage.value
}));

</script>
<template>
  <div>
    <h2>Homepage</h2>

    <div v-if="!loading">
      <!-- this shit is so stupid. fuck u vue what the fuck. 
    how on earth you can fuck up the FOR loop so badly-->
      <Pagination :pages="result.latestPosts.pages" v-bind:current-page="currentPage" @change-page="(newPage: number) => {
      currentPage = newPage;
      router.replace('/page/' + currentPage);
    }" />
      <hr />
      <li v-for="post in result.latestPosts.posts" style=" list-style-type: none;">
        <ShortPost :post="post" />
      </li>
    </div>

    <div v-else-if="loading">
      <h3 style="color: greenyellow">Loading posts...</h3>
    </div>

  </div>
</template>
<style>


.btn_page:disabled {
  color: red;
  background-color: lightgreen;
  cursor: not-allowed;
}
</style>