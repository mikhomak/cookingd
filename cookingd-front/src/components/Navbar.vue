<script setup lang="ts">

import { RouterLink } from 'vue-router'
</script>

<template>
  <nav>
    <h3>Cookingd</h3>
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/my-account">My Account</RouterLink>
    <RouterLink to="/About">About</RouterLink>
    </nav>
</template>

<style scoped>
nav{
  padding-top: 40px;
  text-align: center;
  padding-bottom: 20px;
}
a{
  padding-left: 10px;
}
</style>
