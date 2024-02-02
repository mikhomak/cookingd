<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import ShortPost from '@/components/posts/ShortPost.vue'

const Latest_Post_Query = gql`
  query{
  latestPosts{
    id
    title
    likes
    rating
    text
    shortText
    createdAt
    tags{
      name
    }
  }
}
`

const { result, loading, error } = useQuery(Latest_Post_Query);
</script>
<template>
    <main>
        <h2>Homepage</h2>
        <div v-if="!loading">
            <li v-for="post in result.latestPosts" style=" list-style-type: none;">
                <ShortPost :post="post" />
            </li>
        </div>
        <div v-else-if="error">
            <h3>Oopsie, there was an error!</h3>
        </div>
    </main>
</template>
