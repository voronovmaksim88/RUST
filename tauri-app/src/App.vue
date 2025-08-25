<script setup lang="ts">
import { ref, onMounted } from 'vue'
import PortScanner from './components/PortScanner.vue'
import TheProjectTree from './components/TheProjectTree.vue'

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
    console.log('Fetching from /test_project.json...')
    const response = await fetch('/test_project.json')
    console.log('Response status:', response.status)
    console.log('Response ok:', response.ok)
    console.log('Response headers:', response.headers)
    
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    
    // Сначала получаем текст ответа для отладки
    const responseText = await response.text()
    console.log('Response text (first 100 chars):', responseText.substring(0, 100))
    console.log('Response text length:', responseText.length)
    
    // Проверяем на BOM и другие невидимые символы
    if (responseText.charCodeAt(0) === 0xFEFF) {
      console.log('BOM detected, removing...')
      const cleanText = responseText.slice(1)
      console.log('Clean text (first 100 chars):', cleanText.substring(0, 100))
      
      const data = JSON.parse(cleanText)
      console.log('Parsed data after BOM removal:', data)
      
      if (data && data.project && data.project.name && data.project.author && data.project.log) {
        console.log('Data structure is valid')
        projectData.value = data
        console.log('projectData.value set to:', projectData.value)
      } else {
        console.error('Invalid data structure after BOM removal:', data)
        projectData.value = null
      }
    } else {
      // Обычный парсинг без BOM
      const data = JSON.parse(responseText)
      console.log('Loaded data:', data)
      console.log('Data type:', typeof data)
      console.log('Data keys:', Object.keys(data))
      console.log('Data.project:', data.project)
      
      if (data && data.project && data.project.name && data.project.author && data.project.log) {
        console.log('Data structure is valid')
        projectData.value = data
        console.log('projectData.value set to:', projectData.value)
      } else {
        console.error('Invalid data structure:', data)
        projectData.value = null
      }
    }
  } catch (error) {
    console.error('Error loading project data:', error)
    projectData.value = null
  } finally {
    isLoading.value = false
    console.log('Loading finished. Final projectData.value:', projectData.value)
  }
}

// Функция форматирования даты
const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

onMounted(() => {
  loadProjectData()
})
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
        <TheProjectTree :fileName="fileName" />
      </div>
    </div>

    <!-- Основной контент -->
    <main class="main-content">
      <div v-if="projectData !== null" class="project-details">
        <h1>{{ projectData.project.name }}</h1>
        
        <div class="project-info">
          <div class="info-section">
            <h3>Project Information</h3>
            <div class="info-grid">
              <div class="info-item">
                <label>Name:</label>
                <span>{{ projectData.project.name }}</span>
              </div>
              <div class="info-item">
                <label>Author:</label>
                <span>{{ projectData.project.author }}</span>
              </div>
            </div>
          </div>
          
          <div class="log-section">
            <h3>Project Log</h3>
            <div class="log-entries">
              <div 
                v-for="(record, index) in projectData.project.log.records" 
                :key="index" 
                class="log-entry"
              >
                <div class="log-date">{{ formatDate(record.date) }}</div>
                <div class="log-text">{{ record.text }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div v-else class="loading">
        Loading project data...
      </div>
      
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

/* Стили для отображения свойств проекта */
.project-details {
  margin-bottom: 2rem;
}

.project-details h1 {
  color: #2c3e50;
  margin-bottom: 1.5rem;
  border-bottom: 2px solid #3498db;
  padding-bottom: 0.5rem;
}

.project-info {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.info-section, .log-section {
  background: #f8f9fa;
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid #e9ecef;
}

.info-section h3, .log-section h3 {
  color: #34495e;
  margin: 0 0 1rem 0;
  font-size: 1.2rem;
}

.info-grid {
  display: grid;
  gap: 1rem;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: white;
  border-radius: 4px;
  border: 1px solid #dee2e6;
}

.info-item label {
  font-weight: 600;
  color: #495057;
}

.info-item span {
  color: #6c757d;
  font-weight: 500;
}

.log-entries {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.log-entry {
  padding: 1rem;
  background: white;
  border-radius: 6px;
  border-left: 4px solid #3498db;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.log-date {
  font-size: 0.85rem;
  color: #95a5a6;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.log-text {
  color: #2c3e50;
  line-height: 1.4;
}

.loading {
  text-align: center;
  padding: 2rem;
  color: #6c757d;
  font-style: italic;
}
</style>
