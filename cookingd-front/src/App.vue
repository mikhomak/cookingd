<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from '@/components/Navbar.vue'


</script>

<template>
  <div style="
   display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column; 
  ">
  <header>
    <NavBar/>
  </header>
  <RouterView />
  </div>
</template>

<style scoped>
</style>
