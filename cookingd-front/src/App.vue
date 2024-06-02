<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from '@/components/Navbar.vue'
import { useQuery } from '@vue/apollo-composable';
import { ref } from 'vue';
import router from './router';
import VueCookies from 'vue-cookies'
import { useUserStore } from '@/stores/useUserStore';
import gql from 'graphql-tag';

// @ts-ignore
const tokenFromCookies = VueCookies.get('remember_me');


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
const token_loading = ref(false);
const token_error = ref(null);

if (tokenFromCookies && !userStore.isLoggedIn) {
  const { result, loading: token_loading, error: token_error, onResult } = useQuery(VERIFY_LOGIN_QUERY, () => ({
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
    }
  });

}
</script>

<template>
  <div style="
   display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column; 
  ">
    <header>
      <NavBar />
    </header>
    
    <div v-if="token_loading">
      <h3 style="color: darkorange;">Trying to log you in...</h3>
    </div>

    <div v-else-if="token_error">
      <h3 style="color: red;">Something went wrong with logging you in!</h3>
      <RouterLink to="/login" style="text-align: center;">Go to login</RouterLink>
    </div>

    <main>
    <RouterView />
    </main>
  </div>
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

.post_image {
  max-width: 256px;
  margin-bottom: 15px;
  align-self: center;
}

</style>
