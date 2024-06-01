<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import { useUserStore } from '@/stores/useUserStore';
import ShortPost from '@/components/posts/ShortPost.vue'

const userStore = useUserStore();
const USER_INFO_QUERY = gql`
 query($id: String!){
  user(id: $id){
    name
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
  }
}
`

const { result, loading, error } = useQuery(USER_INFO_QUERY, () => ({
  id: userStore.user.id
}));
console.log(result)
</script>
<template>
  <main>
    <h2>My Account</h2>
    <div v-if="!loading">
      user: {{ result.user.name }}
      <h3>Posts:</h3>
      <li v-for="post in result.user.posts" style=" list-style-type: none;">
        <ShortPost :post="post" />
      </li>
    </div>
    <div v-else-if="error">
      <h3>Oopsie, there was an error!</h3>
    </div>
  </main>
</template>
