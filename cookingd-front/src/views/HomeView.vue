<script  lang="ts">
import ShortPost from '@/components/posts/ShortPost.vue'

import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import { watch } from 'vue'
const CHARACTERS_QUERY = gql`
  query{
  latestPosts{
    id
    title
    tags{
      name
    }
  }
}
`

export default {
  setup () {
    const { result, loading, error } = useQuery(CHARACTERS_QUERY);

    watch(result, value => {
      console.log(value)
    })
    
    return {
      result,
      loading, 
      error
    }
  }
}

</script>

<template>
  <main>
    home page
    <ShortPost title="hehe" />
    <ul v-if="result && result.latestPosts">
    <li v-for="post of result.latestPosts" :key="post.id">
      {{ post.title }} 
        </li>
  </ul>
  </main>
</template>
