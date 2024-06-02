<script setup lang="ts">
import { type Tag} from '@/types/post'
import TagComonent from './TagComponent.vue'

const props = defineProps({
    tags: { type: Object as () => Tag[] }
});
</script>
<template>
    <div style="display: flex; flex-wrap: wrap;">
    <li v-if="props.tags" v-for="tag in props.tags" style="margin-right: 10px; list-style: none;">
        <TagComonent :tag="tag" />
    </li>
    </div>
</template>