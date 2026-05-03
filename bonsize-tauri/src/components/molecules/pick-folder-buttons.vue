<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { platform } from "@tauri-apps/plugin-os";

import MultiButton from "../atoms/multi-button.vue";

defineProps({
    vertical: { type: Boolean, default: true },
});

async function pickFolder() {
    const folder = await open({
        multiple: false,
        directory: true,
    });

    if (folder === null) {
        return;
    }

    window.location.hash = `#/folder-details?path=${folder}`;
}

async function scanAllFiles() {
    const currentPlatform = platform();

    if (currentPlatform === "windows") {
        // TODO: Implement Windows root directory selection logic and handle multiple drives (for example, C:/, D:/, etc.).
    } else {
        window.location.hash = "#/folder-details?path=/";
    }
}
</script>

<template>
    <div class="flex gap-4" :class="[
        vertical ?
            'flex-col items-center' :
            'flex flex-row-reverse'
    ]">
        <MultiButton :class="[vertical ? 'w-64 py-3' : 'h-12']" text="Pick folder" @click="pickFolder" />
        <MultiButton :class="[vertical ? 'w-64 py-3' : 'h-12']" text="Scan all my files" :full="false"
            @click="scanAllFiles" />
    </div>
</template>
