<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>
    
    <GreetForm />
    
    <PortScanner />
  </main>
</template>

<script setup lang="ts">
import GreetForm from './components/GreetForm.vue'
import PortScanner from './components/PortScanner.vue'
</script>

<style>
@import './styles.css';
</style>
