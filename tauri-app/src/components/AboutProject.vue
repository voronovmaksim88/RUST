<script setup lang="ts">
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

interface Props {
  projectData: ProjectData | null
}

const props = defineProps<Props>()

// Функция форматирования даты
const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}
</script>

<template>
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
  
  <div v-else class="empty-state">
    <div class="empty-icon">📁</div>
    <h2>Нет открытого проекта</h2>
    <p>Выберите <strong>File → Открыть</strong> для загрузки проекта</p>
    <p>или <strong>File → Новый</strong> для создания нового проекта</p>
  </div>
</template>

<style scoped>
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

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  text-align: center;
  color: #6c757d;
}

.empty-icon {
  font-size: 5rem;
  margin-bottom: 1.5rem;
  opacity: 0.5;
}

.empty-state h2 {
  color: #495057;
  margin-bottom: 1rem;
}

.empty-state p {
  margin: 0.5rem 0;
  font-size: 1rem;
  line-height: 1.6;
}

.empty-state strong {
  color: #2c3e50;
  font-weight: 600;
}
</style>
