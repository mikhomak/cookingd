<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import ShortPost from '@/components/posts/ShortPost.vue'
import { useUserStore } from '@/stores/useUserStore';
import { ref } from 'vue';
import VueCookies from 'vue-cookies'
import router from '@/router';

const LATEST_POSTS_QUERY = gql`
  query{
  latestPosts{
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

const { result, loading, error } = useQuery(LATEST_POSTS_QUERY);

const tokenFromCookies = VueCookies.VueCookies.get('remember_me');
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

    <li v-if="!loading" v-for="post in result.latestPosts" style=" list-style-type: none;">
      <ShortPost :post="post" />
    </li>

    <div v-else-if="loading">
      <h3 style="color: greenyellow">Loading posts...</h3>
    </div>

  </main>
</template>
