<script setup lang="ts">
import { ref, onMounted, computed, nextTick } from 'vue'
import { invoke } from "@tauri-apps/api/core"

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
    const data = await invoke('load_project_data')
    projectData.value = data
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

const deviceCount = computed(() => {
  if (!projectData.value?.project?.devices) return 0
  return Object.keys(projectData.value.project.devices).length
})

const contextMenu = ref<{
  show: boolean
  x: number
  y: number
  deviceKey: string
}>({
  show: false,
  x: 0,
  y: 0,
  deviceKey: ''
})

const devicesHeaderContextMenu = ref<{
  show: boolean
  x: number
  y: number
}>({
  show: false,
  x: 0,
  y: 0
})

const showCreateDeviceModal = ref(false)

const showContextMenu = (event: MouseEvent, deviceKey: string) => {
  event.preventDefault()
  contextMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY,
    deviceKey
  }

  // Add click listener to hide menu when clicking elsewhere
  nextTick(() => {
    document.addEventListener('click', hideContextMenu)
  })
}

const showDevicesHeaderContextMenu = (event: MouseEvent) => {
  event.preventDefault()
  devicesHeaderContextMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY
  }

  // Add click listener to hide menu when clicking elsewhere
  nextTick(() => {
    document.addEventListener('click', hideDevicesHeaderContextMenu)
  })
}

const hideContextMenu = () => {
  contextMenu.value.show = false
  document.removeEventListener('click', hideContextMenu)
}

const hideDevicesHeaderContextMenu = () => {
  devicesHeaderContextMenu.value.show = false
  document.removeEventListener('click', hideDevicesHeaderContextMenu)
}

const openCreateDeviceModal = () => {
  showCreateDeviceModal.value = true
  hideDevicesHeaderContextMenu()
}

const closeCreateDeviceModal = () => {
  showCreateDeviceModal.value = false
}

const deleteDevice = async (deviceKey: string) => {
  if (!projectData.value?.project?.devices) return

  try {
    // Remove device from local data
    delete projectData.value.project.devices[deviceKey]

    // Save updated data using Tauri command
    const result = await invoke('save_project_data', {
      data: projectData.value
    })

    console.log(`Device ${deviceKey} deleted successfully:`, result)
  } catch (error) {
    console.error('Error deleting device:', error)
    // Reload data to revert changes
    await loadProjectData()
  }

  hideContextMenu()
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
            <div class="tree-header" @click="toggleDevices" @contextmenu="showDevicesHeaderContextMenu">
              <span class="expand-icon" :class="{ 'expanded': isDevicesExpanded }">▶</span>
              <span class="item-icon">🔌</span>
              <span class="item-name">Devices</span>
              <span class="device-count">({{ deviceCount }})</span>
            </div>

            <div v-if="isDevicesExpanded" class="tree-children">
              <div v-if="projectData?.project?.devices && Object.keys(projectData.project.devices).length > 0">
                <div
                  v-for="(device, key) in projectData.project.devices"
                  :key="key"
                  class="tree-item device-detail"
                  @contextmenu="showContextMenu($event, key)"
                >
                  <span class="item-icon">📱</span>
                  <div class="device-info">
                    <span class="device-name">{{ device.name }}</span>
                    <span class="device-type">{{ device.type }}</span>
                  </div>
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

  <!-- Device Context Menu -->
  <div
    v-if="contextMenu.show"
    class="context-menu"
    :style="{
      left: contextMenu.x + 'px',
      top: contextMenu.y + 'px'
    }"
    @click.stop
  >
    <div class="context-menu-item" @click="deleteDevice(contextMenu.deviceKey)">
      <span class="context-menu-icon">🗑️</span>
      <span>Удалить</span>
    </div>
  </div>

  <!-- Devices Header Context Menu -->
  <div
    v-if="devicesHeaderContextMenu.show"
    class="context-menu"
    :style="{
      left: devicesHeaderContextMenu.x + 'px',
      top: devicesHeaderContextMenu.y + 'px'
    }"
    @click.stop
  >
    <div class="context-menu-item" @click="openCreateDeviceModal">
      <span class="context-menu-icon">➕</span>
      <span>Добавить</span>
    </div>
  </div>

  <!-- Create Device Modal -->
  <div v-if="showCreateDeviceModal" class="modal-overlay" @click="closeCreateDeviceModal">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>Создание устройства</h3>
        <button class="modal-close" @click="closeCreateDeviceModal">×</button>
      </div>
      <div class="modal-body">
        <p>Здесь будет форма создания устройства...</p>
        <p>Пока что это заглушка 🚧</p>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" @click="closeCreateDeviceModal">Отмена</button>
        <button class="btn btn-primary">Создать</button>
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
  align-items: flex-start;
  gap: 0.5rem;
  padding: 0.25rem 0.5rem;
  margin-left: 0.5rem;
}

.device-info {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.device-name {
  font-size: 0.9rem;
  color: #495057;
  font-weight: 500;
}

.device-type {
  font-size: 0.75rem;
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

.device-count {
  font-size: 0.8rem;
  color: #6c757d;
  font-weight: 400;
  margin-left: 0.3rem;
}

.device-detail {
  position: relative;
}

.device-detail:hover {
  background-color: #f8f9fa;
  border-radius: 4px;
}

.context-menu {
  position: fixed;
  background: white;
  border: 1px solid #ddd;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  min-width: 120px;
  padding: 4px 0;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  cursor: pointer;
  font-size: 0.9rem;
  color: #495057;
  transition: background-color 0.2s;
}

.context-menu-item:hover {
  background-color: #f8f9fa;
}

.context-menu-item:active {
  background-color: #e9ecef;
}

.context-menu-icon {
  font-size: 0.9rem;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  min-width: 400px;
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid #dee2e6;
  background: #f8f9fa;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.2rem;
  color: #495057;
}

.modal-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #6c757d;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: background-color 0.2s;
}

.modal-close:hover {
  background: #e9ecef;
}

.modal-body {
  padding: 1.5rem;
}

.modal-body p {
  margin: 0.5rem 0;
  color: #6c757d;
}

.modal-footer {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
  padding: 1rem 1.5rem;
  border-top: 1px solid #dee2e6;
  background: #f8f9fa;
}

.btn {
  padding: 0.5rem 1rem;
  border: 1px solid transparent;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background: #545b62;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover {
  background: #0056b3;
}
</style>
