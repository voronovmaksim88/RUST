<script setup lang="ts">
import { ref } from 'vue'
import PortScanner from './components/PortScanner.vue'
import TheProjectTree from './components/TheProjectTree.vue'

const isSidebarCollapsed = ref(false)

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value
}
</script>


<template>
  <div class="app-container">
    <!-- Левая панель с деревом проекта -->
    <div class="sidebar" :class="{ 'collapsed': isSidebarCollapsed }">
      <div class="sidebar-header">
        <h3 v-show="!isSidebarCollapsed">Project Tree</h3>
        <button 
          class="collapse-btn" 
          :class="{ 'collapsed-btn': isSidebarCollapsed }"
          @click="toggleSidebar"
          :title="isSidebarCollapsed ? 'Развернуть' : 'Свернуть'"
        >
          {{ isSidebarCollapsed ? '→' : '←' }}
        </button>
      </div>
      <div class="sidebar-content" v-show="!isSidebarCollapsed">
        <TheProjectTree />
      </div>
    </div>

    <!-- Основной контент -->
    <main class="main-content">

      <PortScanner />
    </main>
  </div>
</template>


<style>
@import './styles.css';

.app-container {
  display: flex;
  height: 100vh;
  width: 100%;
}

.sidebar {
  width: 300px;
  background-color: #f5f5f5;
  border-right: 1px solid #ddd;
  transition: width 0.3s ease;
  overflow: hidden;
}

.sidebar.collapsed {
  width: 60px;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid #ddd;
  background-color: #e9ecef;
}

.sidebar.collapsed .sidebar-header {
  justify-content: center;
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: #333;
}

.collapse-btn {
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 0.5rem;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.2s;
}

.collapse-btn:hover {
  background: #0056b3;
}

.collapsed-btn {
  width: 100%;
  margin: 0;
}

.sidebar-content {
  padding: 1rem;
}

.main-content {
  flex: 1;
  padding: 2rem;
  overflow-y: auto;
}

/* Адаптивность для мобильных устройств */
@media (max-width: 768px) {
  .app-container {
    flex-direction: column;
  }
  
  .sidebar {
    width: 100%;
    height: auto;
  }
  
  .sidebar.collapsed {
    width: 100%;
  }
}
</style>
