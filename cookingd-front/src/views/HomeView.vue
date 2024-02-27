<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import ShortPost from '@/components/posts/ShortPost.vue'
import { useUserStore } from '@/stores/useUserStore';
import { ref } from 'vue';
import VueCookies from 'vue-cookies'

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
    }
  }
}
`

const userStore = useUserStore();

const { result, loading, error } = useQuery(LATEST_POSTS_QUERY);

const tokenFromCookies = VueCookies.get('remember_me');
const token_loading = ref(false);
const token_error = ref(null);
const token_result = ref(null);
if (tokenFromCookies && !userStore.isLoggedIn) {
  const { result: token_result, loading: token_loading, error: token_error } = useQuery(VERIFY_LOGIN_QUERY);
  if(token_result){
    console.log(token_result.login.token);
  }
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

  </main>
</template>
