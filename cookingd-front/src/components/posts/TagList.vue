<script setup lang="ts">
import { type Tag} from '@/types/post'
import TagComonent from './TagComponent.vue'

const props = defineProps({
    tags: { type: Object as () => Tag[] }
});
</script>
<template>
    <div style="display: flex;">
    <li v-if="props.tags" v-for="tag in props.tags" style="margin-right: 10px;">
        <TagComonent :tag="tag" />
    </li>
    </div>
</template>