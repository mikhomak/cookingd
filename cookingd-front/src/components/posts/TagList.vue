<script setup lang="ts">
import { type Tag} from '@/types/post'
import TagComonent from './TagComponent.vue'

const props = defineProps({
    tags: { type: Object as () => Tag[] }
});
</script>
<template>
    <li v-if="props.tags" v-for="tag in props.tags">
        <TagComonent :tag="tag" />
    </li>
</template>