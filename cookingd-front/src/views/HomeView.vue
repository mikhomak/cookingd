<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import ShortPost from '@/components/posts/ShortPost.vue'
import Pagination from '@/components/Pagination.vue'
import { useUserStore } from '@/stores/useUserStore';
import { ref } from 'vue';
import VueCookies from 'vue-cookies'
import router from '@/router';

const currentPage = ref(0);

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

const VERIFY_LOGIN_QUERY = gql`
query ( $token: String!){
  verifyToken(token : $token){
    token
    user {
     id
     name
    }
  }
}
`

const userStore = useUserStore();

const { result, loading, error } = useQuery(LATEST_POSTS_QUERY, () => ({
  page: currentPage.value
}));


// @ts-ignore
const tokenFromCookies = VueCookies.get('remember_me');
const token_loading = ref(false);
const token_error = ref(null);

if (tokenFromCookies && !userStore.isLoggedIn) {
  const { loading: token_loading, error: token_error, onResult } = useQuery(VERIFY_LOGIN_QUERY, () => ({
    token: tokenFromCookies
  }));
  onResult(result => {
    if (!result.loading) {
      userStore.isLoggedIn = true;
      userStore.token = result.data.verifyToken.token;
      userStore.user = {
        id: result.data.verifyToken.user.id,
        name: result.data.verifyToken.user.name
      };
      router.push({ path: '/' })
    }
  });
}

</script>
<template>
  <main>
    <h2>Homepage</h2>

    <div v-if="token_loading">
      <h3 style="color: darkorange;">Trying to log you in...</h3>
    </div>

    <div v-else-if="token_error">
      <h3 style="color: red;">Something went wrong with logging you in!</h3>
      <RouterLink to="/login" style="text-align: center;">Go to login</RouterLink>
    </div>

    <div v-if="!loading">
      <!-- this shit is so stupid. fuck u vue what the fuck. 
    how on earth you can fuck up the FOR loop so badly-->
      <Pagination :pages="result.latestPosts.pages" v-bind:current-page="currentPage"
        @change-page="(newPage: number) => { currentPage = newPage; console.log('asdasd') }" />
      <hr />
      <li v-for="post in result.latestPosts.posts" style=" list-style-type: none;">
        <ShortPost :post="post" />
      </li>
    </div>

    <div v-else-if="loading">
      <h3 style="color: greenyellow">Loading posts...</h3>
    </div>

  </main>
</template>
<style>
@media (min-width: 1020px) {
  main {
    width: 30%;
  }
}

@media (max-width: 700px) {
  main {
    width: 100%;
  }
}

.btn_page:disabled {
  color: red;
  background-color: lightgreen;
  cursor: not-allowed;
}
</style>