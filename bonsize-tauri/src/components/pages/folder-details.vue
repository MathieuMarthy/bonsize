<script setup lang="ts">
import { ref, onMounted, Ref } from "vue";
import { FileModel } from "../../models/FileModel";
import { invoke } from "@tauri-apps/api/core";

const isLoading = ref(true);
const directoryInformations: Ref<FileModel | undefined> = ref(undefined)

onMounted(() => {
    const hash = window.location.hash;
    const urlParams = new URLSearchParams(hash.split("?")[1] || "");
    const folderPath = urlParams.get("path") || "";

    get_directory_informations(folderPath);
});

async function get_directory_informations(path: string) {
    directoryInformations.value = await invoke("scan_directory", { path: path });
    isLoading.value = false;
}
</script>

<template>
    <div>
        <p v-if="isLoading" class="text-text">is loading...</p>
        <p v-else class="text-text">loaded !</p>
    </div>
</template>
