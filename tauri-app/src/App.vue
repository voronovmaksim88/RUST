<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core"
import { open, save } from '@tauri-apps/plugin-dialog'
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
const currentFilePath = ref<string | null>(null)
const fileName = ref<string>('Нет открытого проекта')
const isLoading = ref(false)

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value
}

const loadProjectData = async (filePath: string) => {
  console.log('Starting to load project data from:', filePath)
  isLoading.value = true

  try {
    const data = await invoke('load_project_data', { filePath })
    console.log('Loaded data from Tauri:', data)

    if (data && data.project && data.project.name && data.project.author && data.project.log) {
      console.log('Data structure is valid')
      projectData.value = data
      currentFilePath.value = filePath
      
      // Извлекаем имя файла из полного пути
      const pathParts = filePath.split(/[\\/]/)
      fileName.value = pathParts[pathParts.length - 1]
      
      console.log('projectData.value set to:', projectData.value)
    } else {
      console.error('Invalid data structure:', data)
      projectData.value = null
    }
  } catch (error) {
    console.error('Error loading project data:', error)
    alert('Ошибка при загрузке проекта: ' + error)
    projectData.value = null
  } finally {
    isLoading.value = false
    console.log('Loading finished. Final projectData.value:', projectData.value)
  }
}

const saveProjectData = async (filePath: string) => {
  if (!projectData.value) {
    console.error('No project data to save')
    return
  }

  try {
    const result = await invoke('save_project_data', {
      filePath,
      data: projectData.value
    })
    console.log('Project saved successfully:', result)
    currentFilePath.value = filePath
    
    // Обновляем имя файла
    const pathParts = filePath.split(/[\\/]/)
    fileName.value = pathParts[pathParts.length - 1]
    
    alert('Проект успешно сохранен!')
  } catch (error) {
    console.error('Error saving project data:', error)
    alert('Ошибка при сохранении проекта: ' + error)
  }
}

const handleProjectUpdated = async (updatedData: ProjectData) => {
  projectData.value = updatedData
  
  // Автоматически сохраняем, если файл уже открыт
  if (currentFilePath.value) {
    try {
      await invoke('save_project_data', {
        filePath: currentFilePath.value,
        data: updatedData
      })
      console.log('Project auto-saved')
    } catch (error) {
      console.error('Error auto-saving project:', error)
      alert('Ошибка при автосохранении: ' + error)
    }
  }
}

// Обработчики меню
const handleNewProject = async () => {
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
    currentFilePath.value = null
    fileName.value = 'Новый проект (не сохранен)'
    console.log('New project created')
  }
}

const handleOpenProject = async () => {
  try {
    // Получаем директорию exe файла
    const exeDir = await invoke<string>('get_exe_directory')
    console.log('Exe directory:', exeDir)
    
    // Открываем диалог выбора файла
    const selected = await open({
      multiple: false,
      directory: false,
      defaultPath: exeDir,
      filters: [{
        name: 'JSON Project',
        extensions: ['json']
      }]
    })
    
    if (selected) {
      console.log('Selected file:', selected)
      await loadProjectData(selected as string)
    }
  } catch (error) {
    console.error('Error opening project:', error)
    alert('Ошибка при открытии проекта: ' + error)
  }
}

const handleSaveProject = async () => {
  if (!projectData.value) {
    alert('Нет данных для сохранения')
    return
  }
  
  // Если файл уже был открыт, сохраняем в него
  if (currentFilePath.value) {
    await saveProjectData(currentFilePath.value)
  } else {
    // Если новый проект, вызываем "Сохранить как"
    await handleSaveProjectAs()
  }
}

const handleSaveProjectAs = async () => {
  if (!projectData.value) {
    alert('Нет данных для сохранения')
    return
  }
  
  try {
    // Получаем директорию exe файла
    const exeDir = await invoke<string>('get_exe_directory')
    
    // Открываем диалог сохранения файла
    const selected = await save({
      defaultPath: exeDir + '\\new_project.json',
      filters: [{
        name: 'JSON Project',
        extensions: ['json']
      }]
    })
    
    if (selected) {
      console.log('Save to file:', selected)
      await saveProjectData(selected)
    }
  } catch (error) {
    console.error('Error saving project as:', error)
    alert('Ошибка при сохранении проекта: ' + error)
  }
}

onMounted(() => {
  // Не загружаем проект автоматически
  console.log('App mounted. Waiting for user to open a project.')
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
          <TheProjectTree 
            :fileName="fileName" 
            :projectData="projectData"
            @project-updated="handleProjectUpdated"
          />
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
