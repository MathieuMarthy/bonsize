<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";

import MultiButton from "../atoms/multi-button.vue";

async function pick_folder() {
    const folder = await open({
        multiple: false,
        directory: true,
    });
    
    if (folder === null) {
        return;
    }
    
    window.location.hash = `#/folder-details?path=${folder}`;
}
</script>

<template>
    <div class="flex flex-col items-center gap-4">
        <MultiButton class="w-64" text="Pick folder" @click="pick_folder"/>
        <MultiButton class="w-64" text="Scan all my files" :full="false"/>
    </div>
</template>
