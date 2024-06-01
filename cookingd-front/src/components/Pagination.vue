<script setup lang="ts">
const props = defineProps({
    currentPage: Number,
    pages: Number,
});
const emits = defineEmits<{
    (e: 'changePage', page: number): void
}>()
</script>

<template>
    <h4>pages</h4>
    <div style="display: flex;">
        <li v-for="page in props.pages! + 1" style="margin:5px; list-style-type: none;">
            <button @click="() => { emits('changePage', page - 1) }" v-bind:disabled="props.currentPage === page - 1"
                class="btn_page">
                {{ page - 1 }}
            </button>
        </li>
    </div>
</template>

