import './assets/main.css'

import { createApp, h, provide } from 'vue'
// @ts-ignore
import App from './App.vue'
import router from './router'
import { ApolloClient, ApolloLink, InMemoryCache } from '@apollo/client/core'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { createPinia } from 'pinia'
// @ts-ignore
import createUploadLink from "apollo-upload-client/createUploadLink.mjs";
import { useUserStore } from '@/stores/useUserStore'

const cache = new InMemoryCache()

const pinia = createPinia()


const apolloClient = new ApolloClient({
  cache,
  connectToDevTools: true,
  uri: import.meta.env.VITE_BACKEND_URL,
  link: ApolloLink.from([
    createUploadLink({
      uri: import.meta.env.VITE_BACKEND_URL ,
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
  const headers: { authorization?: string; "Content-Type"?: string } = {};
  const userStore = useUserStore();
  const token = userStore.token;
  if (token) {
    headers["authorization"] = `${token}`;
  }
  return headers;
}