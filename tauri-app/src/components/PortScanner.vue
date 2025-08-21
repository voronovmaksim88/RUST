<template>
  <div>
    <button @click="scanPorts" style="margin-top: 1rem;">
      Scan COM Ports
    </button>
    <pre>{{ portsResult }}</pre>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core"

const portsResult = ref('')

async function scanPorts() {
  portsResult.value = "Scanning..."
  try {
    const result = await invoke<{ ports: number[] }>("scan_available_ports_cmd")
    if (result.ports.length === 0) {
      portsResult.value = "No COM ports found."
    } else {
      portsResult.value = `Found ports: ${result.ports.map((p) => `COM${p}`).join(", ")}`
    }
  } catch (e) {
    portsResult.value = `Error: ${e}`
  }
}
</script>
