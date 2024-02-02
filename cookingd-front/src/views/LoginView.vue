<script setup lang="ts">
import { useUserStore } from '@/stores/useUserStore';
import { useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { ref } from 'vue';

const LOGIN_MUTATION = gql`
mutation login($input: LoginInput!){
  login(loginInput: $input){
    token
    user {
        id
    }
  }
}
`;

const userStore = useUserStore();
const email = ref('');
const password = ref('');

const { mutate: login, onDone, onError, loading, error } = useMutation(LOGIN_MUTATION,
    () => ({
        variables: {
            input: {
                email: email.value,
                password: password.value
            }
        },
    })
)

onDone((data) => {
    userStore.isLoggedIn = true;
    userStore.token = data.data.token;
    userStore.user = data.data.user;
})

onError(errors => {
    userStore.isLoggedIn = false;
    userStore.token = null;
    userStore.user = null;
})

</script>
<template>
    <main>
        <h2>Login</h2>
        <div v-if="!loading">

            <h3 v-if="error" style="color: red">{{ error.message }}</h3>

            <div>
                <label for="input_email">email</label>
                <br />
                <input id="input_email" type="email" v-model="email" />
            </div>

            <div>
                <label for="input_password">password</label>
                <br />
                <input id="input_password" type="text" v-model="password" />
            </div>

            <div>
                <button @click="login()" id="btn_submit">Submit</button>
            </div>
        </div>

        <div v-else>
            loading...
        </div>

    </main>
</template>

<style scoped>
div {
    padding-bottom: 10px;
}
</style>
