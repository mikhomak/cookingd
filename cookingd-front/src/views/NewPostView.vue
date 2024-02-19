<script setup lang="ts">
import { useUserStore } from '@/stores/useUserStore';
import { useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { ref } from 'vue';

const title = ref('');
const text = ref('');
const ingridents = ref('');
const rating = ref(5);
const tags = ref([])
const image = ref(null);
const userStore = useUserStore();
const { mutate: createPost, loading, onDone, error } = useMutation(gql`
    mutation createNewPost($post_input: PostCreationInput!){
    createPost(postInput: $post_input){
            id
        }
    } 
`, () => ({
    variables: {
        post_input: {
            title: title.value,
            text: text.value,
            rating: rating.value,
            tags: tags.value,
            mainImage: image.value,
        }
    },
    context:{
        hasUpload: image.value !== null,
    },
}));

const new_post_created = ref(false);
onDone(data => {
    new_post_created.value = true;
});

async function uploadPhoto({ target }) {
    image.value = target.files[0];
}


</script>

<template>
    <main style="">
        <h2 style="margin: auto; text-align: center; padding-bottom: 20px;">Registration</h2>
        <div v-if="new_post_created" style="width: 50%; margin: auto; text-align: center;">
            <h4 style="color: green;  padding-bottom: 20px;">
                Post has been created!
            </h4>
            <RouterLink to="/" style="text-align: center;">Go to homepage</RouterLink>
        </div>
        <div v-else-if="!loading">
            <form>
                <div>
                    <label for="input_title">title</label>
                    <br />
                    <input id="input_title" type="text" v-model="title" />
                </div>

                <div>
                    <label for="input_text">text</label>
                    <br />
                    <textarea id="input_text" v-model="text"></textarea>
                </div>

                <div>
                    <label for="input_rating">rating</label>
                    <br />
                    <input id="input_rating" type="number" v-model="rating" />
                </div>

                <div>
                    <label for="input_image"> image</label>
                    <input id="input_image" type="file" accept="image/jpeg,image/png,image/jpg"  @change="uploadPhoto"/>
                </div>

                <div v-if="error" style="color: red; width: 50%; margin: auto;">
                    There was an error!
                    <p>
                        Error is [{{ error.message }}]
                    </p>
                </div>
                <div>
                    <button id="btn_submit" @click="createPost()" :disabled="title === ''">
                        Submit
                    </button>
                </div>
            </form>
        </div>
        <div v-else-if="loading">loading...</div>
    </main>
</template>

<style scoped>
div {
    padding-bottom: 10px;
}
</style>
