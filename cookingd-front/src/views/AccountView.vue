<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import { useUserStore } from '@/stores/useUserStore';
import Pagination from '@/components/Pagination.vue'
import ShortPost from '@/components/posts/ShortPost.vue'
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import router from '@/router';

const currentPage = ref(0);
const route = useRoute();
if (route.params.page) {
  //@ts-ignore
  currentPage.value = parseInt(route.params.page)
}

const userStore = useUserStore();
const USER_INFO_QUERY = gql`
query ($id: String!, $page: Int!) {
  user(id: $id) {
    name
    posts(page: $page) {
      posts {
        id
        title
        likes
        rating
        text
        shortText
        createdAt
        mainImageUrl
        tags {
          name
        }
        user {
          name
        }
      }
      pages
    }
  }
}
`

const { result, loading, error } = useQuery(USER_INFO_QUERY, () => ({
  id: userStore.user.id,
  page: currentPage.value
}));

</script>
<template>
  <main>
    <h2>My Account</h2>
    <div v-if="!loading">
      user: {{ result.user.name }}
      <h3>Posts:</h3>
      <Pagination :pages="result.user.posts.pages" v-bind:current-page="currentPage" @change-page="(newPage: number) => {
      currentPage = newPage;
      router.replace(`/my-account/${result.user.name}/page/${currentPage}`);
    }" />
      <hr />
      <li v-for="post in result.user.posts.posts" style=" list-style-type: none;">
        <ShortPost :post="post" />
      </li>
    </div>
    <div v-else-if="error">
      <h3>Oopsie, there was an error!</h3>
    </div>
  </main>
</template>
