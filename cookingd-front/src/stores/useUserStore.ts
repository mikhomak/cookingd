import { defineStore } from "pinia"
import { ref, computed } from "vue"

export const useUserStore = defineStore('user', () => {
  const token = ref(null)
  const user = ref(null)
  const isLoggedIn = ref(false);

  return { token, user, isLoggedIn }
})