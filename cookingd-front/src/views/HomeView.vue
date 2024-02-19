<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import ShortPost from '@/components/posts/ShortPost.vue'

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

const { result, loading, error } = useQuery(LATEST_POSTS_QUERY);

</script>
<template>
  <main>
    <h2>Homepage</h2>
      <li v-if="!loading" v-for="post in result.latestPosts" style=" list-style-type: none;">
        <ShortPost :post="post" />
      </li>
  </main>
</template>
