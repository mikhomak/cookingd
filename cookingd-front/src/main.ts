import './assets/main.css'

import { createApp, h, provide } from 'vue'
import App from './App.vue'
import router from './router'
import { ApolloClient, ApolloLink, InMemoryCache } from '@apollo/client/core'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { createPinia } from 'pinia'
import createUploadLink from "apollo-upload-client/createUploadLink.mjs";
import { useUserStore } from '@/stores/useUserStore'

const cache = new InMemoryCache()

const pinia = createPinia()


const apolloClient = new ApolloClient({
  cache,
  connectToDevTools: true,
  uri: 'http://localhost:8080/',
  link: ApolloLink.from([
    createUploadLink({
      uri: 'http://localhost:8080/',
      fetch: (uri: RequestInfo, options: RequestInit) => {
        options.headers = { ...getHeaders() };
        return fetch(uri, options);
      },
    })
  ]),
});



const app = createApp({
  setup() {
    provide(DefaultApolloClient, apolloClient)
  },

  render: () => h(App),
})

app.use(router);
app.use(pinia);

app.mount('#app');


function getHeaders() {
  const headers: { login?: string; "Content-Type"?: string } = {};
  const userStore = useUserStore();
  const token = userStore.token;
  console.log(token)
  if (token) {
    headers["login"] = `${token}`;
  }
  return headers;
}