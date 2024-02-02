<script setup lang="ts">
import { useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'

const { mutate: createUser } = useMutation(gql`
    mutation{
    createUser(userInput:{
        name: "poopy"
        email: "kaki"
        password: "password"
        consent: true
        }){
            id
        }
    } 
`)


</script>
<template>
    <main>
        <h2>Registration</h2>
        <div>
            <form>
                <div>
                    <label for="input_email">email</label>
                    <br />
                    <input id="input_email" type="email" />
                </div>

                <div>
                    <label for="input_name">name</label>
                    <br />
                    <input id="input_name" type="text" />
                </div>

                <div>
                    <label for="input_password">password</label>
                    <br />
                    <input id="input_password" type="text" />
                </div>

                <div>
                    <label for="input_confirm_password">confirm password</label>
                    <br />
                    <input id="input_confirm_password" type="text" />
                </div>

                <div>
                    <p>Conesnt bla bla bla bla</p>
                    <input id="input_consent" type="checkbox" />
                    <label for="input_consent"> i consent</label>
                </div>

                <div>
                    <button id="btn_submit">Submit</button>
                </div>
            </form>
        </div>
    </main>
</template>

<style scoped>
div {
    padding-bottom: 10px;
}
</style>
