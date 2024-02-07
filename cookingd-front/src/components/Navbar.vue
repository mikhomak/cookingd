<script setup lang="ts">

import { RouterLink } from 'vue-router'

import { useUserStore } from '@/stores/useUserStore.ts'
import router from '@/router';

const userStore = useUserStore();
function logout() {
  userStore.isLoggedIn = false;
  userStore.token = null;
  userStore.user = null;
  router.push({ path: '/' })
}
</script>

<template>
  <nav>
    <h3>Cookingd</h3>
    <RouterLink to="/">Home</RouterLink>
    <RouterLink v-if="userStore.isLoggedIn" to="/my-account">My Account</RouterLink>
    <RouterLink to="/About">About</RouterLink>
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
