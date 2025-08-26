<script setup lang="ts">
import { ref, onMounted } from 'vue'

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
  devices: Record<string, any>
}

interface ProjectData {
  project: Project
}

interface Props {
  fileName: string
}

defineProps<Props>()
const projectData = ref<ProjectData | null>(null)
const isProjectExpanded = ref(true)
const isDevicesExpanded = ref(true)

const loadProjectData = async () => {
  try {
    const response = await fetch('/test_project.json')
    if (response.ok) {
      projectData.value = await response.json()
    }
  } catch (error) {
    console.error('Error loading project data:', error)
  }
}

const toggleProject = () => {
  isProjectExpanded.value = !isProjectExpanded.value
}

const toggleDevices = () => {
  isDevicesExpanded.value = !isDevicesExpanded.value
}

onMounted(() => {
  loadProjectData()
})
</script>

<template>
  <div class="project-tree">
    <div class="tree-container">
      <!-- Проект -->
      <div class="tree-item project-item">
        <div class="tree-header" @click="toggleProject">
          <span class="expand-icon" :class="{ 'expanded': isProjectExpanded }">▶</span>
          <span class="item-icon">📁</span>
          <!-- Название проекта -->
          <span class="item-name">{{ fileName || 'Loading...' }}</span>
        </div>
        
        <div v-if="isProjectExpanded" class="tree-children">

          <!-- Устройства -->
          <div class="tree-item devices-item">
            <div class="tree-header" @click="toggleDevices">
              <span class="expand-icon" :class="{ 'expanded': isDevicesExpanded }">▶</span>
              <span class="item-icon">🔌</span>
              <span class="item-name">Devices</span>
            </div>
            
            <div v-if="isDevicesExpanded" class="tree-children">
              <div v-if="projectData?.project?.devices && Object.keys(projectData.project.devices).length > 0">
                <div 
                  v-for="(device, key) in projectData.project.devices" 
                  :key="key" 
                  class="tree-item device-detail"
                >
                  <span class="item-icon">📱</span>
                  <span class="item-name">{{ device.name }}</span>
                  <span class="device-type">({{ device.type }})</span>
                </div>
              </div>
              <div v-else class="tree-item empty-item">
                <span class="item-icon">📱</span>
                <span class="item-name">No devices</span>
              </div>
            </div>
          </div>
         </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.project-tree {
  padding: 0;
}

.tree-container {
  margin: 0;
}

.tree-item {
  margin: 0;
}

.tree-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.2s;
  user-select: none;
}

.tree-header:hover {
  background-color: #e9ecef;
}

.tree-children {
  margin-left: 1.5rem;
  border-left: 1px solid #dee2e6;
  padding-left: 0.5rem;
}

.expand-icon {
  font-size: 0.8rem;
  color: #6c757d;
  transition: transform 0.2s;
  width: 1rem;
  text-align: center;
}

.expand-icon.expanded {
  transform: rotate(90deg);
}

.item-icon {
  font-size: 1.1rem;
}

.item-name {
  font-size: 0.9rem;
  color: #495057;
  font-weight: 500;
}

.project-item .item-name {
  font-weight: 600;
  color: #2c3e50;
}

.devices-item .item-name {
  color: #6f42c1;
}

.empty-item .item-name {
  color: #6c757d;
  font-style: italic;
}

.device-detail {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0.5rem;
  margin-left: 0.5rem;
}

.device-type {
  font-size: 0.8rem;
  color: #6c757d;
  font-style: italic;
}

.file-item .item-icon {
  font-size: 1.2rem;
}

.file-item .item-name {
  font-size: 0.9rem;
  color: #495057;
  font-weight: 500;
}
</style>
