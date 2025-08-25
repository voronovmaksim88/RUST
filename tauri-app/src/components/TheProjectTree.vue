<script setup lang="ts">
interface Props {
  fileName: string
}

defineProps<Props>()
</script>

<template>
  <div class="project-tree">
    <div class="tree-container">
      <div class="file-item">
        <span class="file-icon">📄</span>
        <span class="file-name">{{ fileName }}</span>
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

.file-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.file-item:hover {
  background-color: #e9ecef;
}

.file-icon {
  font-size: 1.2rem;
}

.file-name {
  font-size: 0.9rem;
  color: #495057;
  font-weight: 500;
}
</style>
