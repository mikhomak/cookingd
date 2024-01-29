<script setup lang="ts">
import { type Tag } from '@/types/post'
const props = defineProps({
    tag: { type: Object as () => Tag }
});
</script>
<template>
    <div v-if="tag" style="
        background-color: khaki; 
        width: fit-content;
        padding: 3px;
        border: 1px solid black;
        ">
        {{ tag.name }}
    </div>
</template>