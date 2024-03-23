<script setup lang="ts">
import router from '@/router';
import { useUserStore } from '@/stores/useUserStore';
import { useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { ref } from 'vue';
import VueCookies from 'vue-cookies'
const LOGIN_MUTATION = gql`
mutation login($input: LoginInput!){
  login(loginInput: $input){
    token
    user {
        id
        name
    }
  }
}
`;

const userStore = useUserStore();
const email = ref('');
const password = ref('');
const rememberMe = ref(false);

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

onDone((result) => {
    userStore.isLoggedIn = true;
    userStore.token = result.data.login.token;
    userStore.user = {
        id: result.data.login.user.id,
        name: result.data.login.user.name
    };
    if (rememberMe.value) {
// @ts-ignore
        VueCookies.set('remember_me', result.data.login.token, "30d");
    }
    router.push({ path: '/' })
})

onError(errors => {
    userStore.isLoggedIn = false;
    userStore.token = null;
    userStore.user = {};
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
                <input id="input_password" type="password" v-model="password" />
            </div>

            <div>
                <label for="input_remember_me">remember me?</label>
                <br />
                <input id="input_remember_me" type="checkbox" v-model="rememberMe" />
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
