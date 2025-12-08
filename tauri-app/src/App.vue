<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core"
import MenuBar from './components/MenuBar.vue'
import TheProjectTree from './components/TheProjectTree.vue'
import AboutProject from './components/AboutProject.vue'

interface LogRecord {
  date: string
  text: string
}

interface ProjectLog {
  records: LogRecord[]
}

interface Project {
  name: string
  author: string
  log: ProjectLog
  devices?: Record<string, any>
}

interface ProjectData {
  project: Project
}

const isSidebarCollapsed = ref(false)
const projectData = ref<ProjectData | null>(null)
const fileName = ref('test_project.json')
const isLoading = ref(false)

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value
}

const loadProjectData = async () => {
  console.log('Starting to load project data...')
  isLoading.value = true

  try {
    const data = await invoke('load_project_data')
    console.log('Loaded data from Tauri:', data)

    if (data && data.project && data.project.name && data.project.author && data.project.log) {
      console.log('Data structure is valid')
      projectData.value = data
      console.log('projectData.value set to:', projectData.value)
    } else {
      console.error('Invalid data structure:', data)
      projectData.value = null
    }
  } catch (error) {
    console.error('Error loading project data:', error)
    projectData.value = null
  } finally {
    isLoading.value = false
    console.log('Loading finished. Final projectData.value:', projectData.value)
  }
}

const saveProjectData = async () => {
  if (!projectData.value) {
    console.error('No project data to save')
    return
  }

  try {
    const result = await invoke('save_project_data', {
      data: projectData.value
    })
    console.log('Project saved successfully:', result)
    alert('Проект успешно сохранен!')
  } catch (error) {
    console.error('Error saving project data:', error)
    alert('Ошибка при сохранении проекта: ' + error)
  }
}

// Обработчики меню
const handleNewProject = () => {
  const confirmed = confirm('Создать новый проект? Несохраненные данные будут потеряны.')
  if (confirmed) {
    projectData.value = {
      project: {
        name: 'Новый проект',
        author: 'Автор',
        devices: {},
        log: {
          records: [
            {
              date: new Date().toISOString().split('T')[0],
              text: 'Проект создан'
            }
          ]
        }
      }
    }
    console.log('New project created')
  }
}

const handleOpenProject = () => {
  loadProjectData()
  console.log('Project opened')
}

const handleSaveProject = () => {
  saveProjectData()
}

const handleSaveProjectAs = () => {
  // TODO: Реализовать диалог выбора файла
  alert('Функция "Сохранить как..." будет реализована позже')
  console.log('Save project as...')
}

onMounted(() => {
  loadProjectData()
})
</script>

<template>
  <div class="app-wrapper">
    <!-- Верхнее меню -->
    <MenuBar
      @new-project="handleNewProject"
      @open-project="handleOpenProject"
      @save-project="handleSaveProject"
      @save-project-as="handleSaveProjectAs"
    />
    
    <!-- Основной контейнер -->
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
          <TheProjectTree :fileName="fileName" />
        </div>
      </div>

      <!-- Основной контент -->
      <main class="main-content">
        <AboutProject :projectData="projectData" />
      </main>
    </div>
  </div>
</template>

<style>
@import './styles.css';

.app-wrapper {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
}

.app-container {
  display: flex;
  flex: 1;
  height: calc(100vh - 40px);
  width: 100%;
  overflow: hidden;
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
