<script setup lang="ts">
import { useMutation } from '@vue/apollo-composable'
import { ref, type Ref } from 'vue';
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'

const title = ref('');
const text = ref('');
const ingridents = ref('');
const input_tag = ref('');
const rating = ref(1);
const tags: Ref<Set<String>> = ref(new Set())
const image = ref(null);


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
            tags: [...tags.value.values()],
            mainImage: image.value,
        }
    },
    context: {
        hasUpload: image.value !== null,
    },
}));

const new_post_created = ref(false);

onDone(data => {
    new_post_created.value = true;
});

// @ts-ignore
async function uploadPhoto({ target }) {
    image.value = target.files[0];
}

function setRating(newRating: number, event: Event) {
    event.preventDefault();
    rating.value = newRating;
}

// tags


const ALL_TAGS_QUERY = gql`
  query{
  allTags{
    name
    }
  }
`

const { result: tag_result, loading: tag_loading } = useQuery(ALL_TAGS_QUERY, {
    fetchPolicy: 'no-cache'
});

</script>

<template>
    <main style="">
        <h2 style="margin: auto; text-align: center; padding-bottom: 20px;">Create new post</h2>
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
                    <button class="btn_rating" id="rating_1" v-bind:disabled="rating === 1"
                        @click="setRating(1, $event)">1</button>
                    <button class="btn_rating" id="rating_2" v-bind:disabled="rating === 2"
                        @click="setRating(2, $event)">2</button>
                    <button class="btn_rating" id="rating_3" v-bind:disabled="rating === 3"
                        @click="setRating(3, $event)">3</button>
                    <button class="btn_rating" id="rating_4" v-bind:disabled="rating === 4"
                        @click="setRating(4, $event)">4</button>
                    <button class="btn_rating" id="rating_5" v-bind:disabled="rating === 5"
                        @click="setRating(5, $event)">5</button>
                </div>

                <div>
                    <label for="input_tag">Tags</label>
                    <ul v-if="!tag_loading && tag_result !== undefined"
                        style="margin-top: 10px; columns:2; padding-left: 0;">
                        <li v-for="tag in tag_result.allTags" style="
                        margin: 7px 0 7px;
                        list-style-type: none;">
                            <span style="
                        color:cadetblue;
                        background-color: azure; 
                        padding: 3px;
                        border: 1px solid black;
                        " @click="(event) => {
            event.preventDefault();
            tags.add(tag.name);
        }">{{ tag.name }}</span>
                        </li>
                    </ul>
                    <div v-else-if="tag_loading">
                        <h3 style="color: greenyellow">Loading tags...</h3>
                    </div>


                    <li v-if="tags.size !== 0" v-for="tag in tags" style=" list-style-type: none;">
                        <span>{{ tag }}</span><button @click="(event) => {
            event.preventDefault();
            tags.delete(tag);
        }">X</button>
                    </li>
                    <br />
                    <input id="input_tag" type="text" v-model="input_tag" />
                    <button id="btn_tag_submit" @click="(event) => {
            event.preventDefault();
            tags.add(input_tag);
            input_tag = '';
        }" :disabled="input_tag === ''">add tag</button>
                </div>

                <div>
                    <label for="input_image"> image</label>
                    <input id="input_image" type="file" accept="image/jpeg,image/png,image/jpg" @change="uploadPhoto" />
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

.btn_rating {
    margin: 3px;
}

.btn_rating:disabled {
    color: red;
    background-color: lightgreen;
}
</style>
