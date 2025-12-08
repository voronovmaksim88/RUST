<script setup lang="ts">
import { ref, nextTick } from 'vue'

interface Emits {
  (e: 'new-project'): void
  (e: 'open-project'): void
  (e: 'save-project'): void
  (e: 'save-project-as'): void
}

const emit = defineEmits<Emits>()

const showFileMenu = ref(false)

const toggleFileMenu = () => {
  showFileMenu.value = !showFileMenu.value
  
  if (showFileMenu.value) {
    nextTick(() => {
      document.addEventListener('click', hideFileMenu)
    })
  }
}

const hideFileMenu = () => {
  showFileMenu.value = false
  document.removeEventListener('click', hideFileMenu)
}

const handleNewProject = () => {
  emit('new-project')
  hideFileMenu()
}

const handleOpenProject = () => {
  emit('open-project')
  hideFileMenu()
}

const handleSaveProject = () => {
  emit('save-project')
  hideFileMenu()
}

const handleSaveProjectAs = () => {
  emit('save-project-as')
  hideFileMenu()
}
</script>

<template>
  <div class="menu-bar">
    <div class="menu-item" @click.stop="toggleFileMenu">
      <span class="menu-label">File</span>
      
      <div v-if="showFileMenu" class="dropdown-menu" @click.stop>
        <div class="dropdown-item" @click="handleNewProject">
          <span class="item-icon">📄</span>
          <span>Новый</span>
          <span class="shortcut">Ctrl+N</span>
        </div>
        
        <div class="dropdown-item" @click="handleOpenProject">
          <span class="item-icon">📂</span>
          <span>Открыть</span>
          <span class="shortcut">Ctrl+O</span>
        </div>
        
        <div class="dropdown-divider"></div>
        
        <div class="dropdown-item" @click="handleSaveProject">
          <span class="item-icon">💾</span>
          <span>Сохранить</span>
          <span class="shortcut">Ctrl+S</span>
        </div>
        
        <div class="dropdown-item" @click="handleSaveProjectAs">
          <span class="item-icon">💾</span>
          <span>Сохранить как...</span>
          <span class="shortcut">Ctrl+Shift+S</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.menu-bar {
  display: flex;
  align-items: center;
  background: #2c3e50;
  color: white;
  padding: 0;
  height: 40px;
  user-select: none;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  z-index: 100;
}

.menu-item {
  position: relative;
  padding: 0.75rem 1rem;
  cursor: pointer;
  transition: background-color 0.2s;
  height: 100%;
  display: flex;
  align-items: center;
}

.menu-item:hover {
  background: #34495e;
}

.menu-label {
  font-size: 0.9rem;
  font-weight: 500;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  background: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-width: 220px;
  padding: 4px 0;
  margin-top: 2px;
  z-index: 1000;
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.6rem 1rem;
  cursor: pointer;
  color: #333;
  font-size: 0.9rem;
  transition: background-color 0.2s;
}

.dropdown-item:hover {
  background: #f8f9fa;
}

.dropdown-item:active {
  background: #e9ecef;
}

.item-icon {
  font-size: 1rem;
  width: 20px;
  text-align: center;
}

.shortcut {
  margin-left: auto;
  font-size: 0.75rem;
  color: #6c757d;
  font-family: monospace;
}

.dropdown-divider {
  height: 1px;
  background: #e9ecef;
  margin: 4px 0;
}

/* Анимация появления меню */
.dropdown-menu {
  animation: slideDown 0.15s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>

