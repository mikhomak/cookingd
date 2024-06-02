<script setup lang="ts">

import { RouterLink } from 'vue-router'

import { useUserStore } from '@/stores/useUserStore.ts'
import router from '@/router';
import VueCookies from 'vue-cookies'

const userStore = useUserStore();
function logout() {
  // @ts-ignore
  VueCookies.remove('remember_me');
  userStore.isLoggedIn = false;
  userStore.token = null;
  userStore.user = {};
  router.push({ path: '/' })
}
</script>

<template>
  <nav>
    <h3>Cookingd</h3>
    <h4>your personal cooking diary</h4>
    <RouterLink to="/" >Home</RouterLink>
    <router-link v-if="userStore.isLoggedIn" :to="`/my-account/${userStore.user.name}`">My Account [{{ userStore.user.name }}] </router-link>
    <RouterLink to="/about">About</RouterLink>
  </nav>
  <nav v-if="userStore.isLoggedIn" style="padding-top: 0;">
    <RouterLink to="/new-post">New post</RouterLink>
    <a href="#" @click="logout()">Logout</a>
  </nav>
  <nav v-else style="padding-top: 0;">
    <RouterLink to="/login">Login</RouterLink>
    <RouterLink to="/register">Register</RouterLink>
  </nav>
</template>

<style scoped>
nav {
  padding-top: 40px;
  text-align: center;
  padding-bottom: 20px;
}

a {
  padding-left: 10px;
}
</style>
