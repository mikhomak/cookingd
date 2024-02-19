import { defineStore } from "pinia"
import { ref } from "vue"

export const useUserStore = defineStore('user', () => {
  const token = ref(null)
  const user = ref<{id?: String}>({
    id: undefined
  })
  const isLoggedIn = ref(false);

  return { token, user, isLoggedIn }
})