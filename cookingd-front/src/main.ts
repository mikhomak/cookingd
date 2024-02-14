import './assets/main.css'

import { createApp, h, provide } from 'vue'
import App from './App.vue'
import router from './router'
import { ApolloClient, ApolloLink, InMemoryCache } from '@apollo/client/core'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { createPinia } from 'pinia'
import { createUploadLink } from 'apollo-upload-client'

const cache = new InMemoryCache()

const apolloClient = new ApolloClient({
  cache,
  uri: 'http://localhost:8080/',
  connectToDevTools: true,
  link: ApolloLink.from([
    createUploadLink({
      uri: '/'
    })
  ])
});

const pinia = createPinia()


const app = createApp({
  setup() {
    provide(DefaultApolloClient, apolloClient)
  },

  render: () => h(App),
})

app.use(router);
app.use(pinia);

app.mount('#app');
