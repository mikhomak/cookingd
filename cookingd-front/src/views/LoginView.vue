<script setup lang="ts">
import { useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'


const { mutate: login, onDone , onError} = useMutation(gql`
mutation login($input: LoginInput!){
  login(loginInput: $input)
}
`, { 
    variables: {
        input: {
            email: 'kaki',
            password: 'poopi'
        }
    }
})

onDone((data) => {
    console.log(data);
})
onError((data)=>{
    console.log("error")
    console.log(data.clientErrors)
    console.log(data.extraInfo)
    console.log(data.graphQLErrors)
    console.log(data.message)
    console.log(data.name)
    console.log(data.protocolErrors)
})

</script>
<template>
    <main>
        <h2>Login</h2>
        <div>
            <form>
                <div>
                    <label for="input_email">email</label>
                    <br />
                    <input id="input_email" type="email" />
                </div>

                <div>
                    <label for="input_password">password</label>
                    <br />
                    <input id="input_password" type="text" />
                </div>

                <div>
                    <button @click="login()" id="btn_submit">Submit</button>
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
