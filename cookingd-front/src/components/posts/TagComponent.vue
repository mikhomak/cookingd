<script setup lang="ts">
import { type Tag } from '@/types/post'
const props = defineProps({
    tag: { type: Object as () => Tag }
});
</script>
<template>
    <div v-if="tag">
        {{ tag.name }}
    </div>
</template>