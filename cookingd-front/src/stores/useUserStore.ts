import { defineStore } from "pinia"
import { ref } from "vue"

export const useUserStore = defineStore('user', () => {
  const token = ref(null)
  const user = ref<{ id?: String, name?: String }>({
    id: undefined,
    name: undefined
  })
  const isLoggedIn = ref(false);

  return { token, user, isLoggedIn }
})