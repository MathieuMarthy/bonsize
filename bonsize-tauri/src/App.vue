<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const path = ref("");
const result = ref({});

async function scan_directory(path: string) {
  result.value = await invoke("scan_directory", { path: path });
}
</script>

<template>
  <main>
    <p>Salut !</p>

    <label>Path</label>
    <input v-model="path" />

    <button @click="() => scan_directory(path)">scan</button>

    <span>{{ result }}</span>
  </main>
</template>
