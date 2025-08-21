<template>
  <form class="row" @submit.prevent="greet">
    <input 
      v-model="name" 
      placeholder="Enter a name..." 
      type="text"
    />
    <button type="submit">Greet</button>
  </form>
  <p>{{ greetMsg }}</p>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core"

const name = ref('')
const greetMsg = ref('')

async function greet() {
  if (name.value) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", {
      name: name.value,
    })
  }
}
</script>
