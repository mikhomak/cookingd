<script setup lang="ts">
import { useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { computed, ref } from 'vue';

const email = ref('');
const name = ref('');
const password = ref('');
const consent = ref(false);

const { mutate: createUser, loading, onDone, error } = useMutation(gql`
    mutation register($registrationInput: UserRegistrationInput!){
    createUser(userInput: $registrationInput){
            id
        }
    } 
`, () => ({
    variables: {
        registrationInput: {
            name: name.value,
            email: email.value,
            password: password.value,
            consent: consent.value
        }
    }
}));

onDone(data => {
    registration_complete.value = true;
});
const confirm_password = ref('');
const registration_complete = ref(false);
</script>
<template>
    <main style="">
        <h2 style="margin: auto; text-align: center; padding-bottom: 20px;">Registration</h2>
        <div v-if="registration_complete" style="width: 50%; margin: auto; text-align: center;">
            <h4 style="color: green;  padding-bottom: 20px;">Registration is complete! You can now go to login page and
                access your account!
            </h4>
            <RouterLink to="/login" style="text-align: center;">Go to login</RouterLink>
        </div>
        <div v-else-if="!loading">
            <form>
                <div>
                    <label for="input_email">email</label>
                    <br />
                    <input id="input_email" type="email" v-model="email" />
                </div>

                <div>
                    <label for="input_name">name</label>
                    <br />
                    <input id="input_name" type="text" v-model="name" />
                </div>

                <div>
                    <label for="input_password">password</label>
                    <br />
                    <input id="input_password" type="text" v-model="password" />
                </div>

                <div>
                    <label for="input_confirm_password">confirm password</label>
                    <br />
                    <input id="input_confirm_password" type="text" v-model="confirm_password" />
                </div>

                <div>
                    <p>Conesnt bla bla bla bla</p>
                    <input id="input_consent" type="checkbox" v-model="consent" />
                    <label for="input_consent"> i consent</label>
                </div>

                <div v-if="error" style="color: red; width: 50%; margin: auto;">
                    There was an error!
                    <p>
                        Error is [{{ error.message }}]
                    </p>
                </div>
                <div>
                    <button id="btn_submit" @click="createUser()" :disabled="!(password !== '' && password == confirm_password)
                        || !consent
                        || email === ''
                        || name === ''
                        ">Submit</button>
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
