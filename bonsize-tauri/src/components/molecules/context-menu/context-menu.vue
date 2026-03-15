<script setup lang="ts">
import { openPath } from "@tauri-apps/plugin-opener";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { onMounted, onUnmounted } from "vue";
import { useContextMenu } from "./useContextMenu";

const { showContextMenu, menuX, menuY, currentFile, closeContextMenu } = useContextMenu();

function handleAction(action: string) {
    if (currentFile.value === null) {
        return;
    }

    switch (action) {
        case "open-folder":
            openPath(currentFile.value.path);
            break;

        case "copy-path":
            writeText(currentFile.value.path);
            break;

        default:
            break;
    }
    closeContextMenu();
}

onMounted(() => {
    window.addEventListener("click", closeContextMenu);
    window.addEventListener("scroll", closeContextMenu, true);
});

onUnmounted(() => {
    window.removeEventListener("click", closeContextMenu);
    window.removeEventListener("scroll", closeContextMenu, true);
});
</script>

<template>
    <div v-if="showContextMenu" class="fixed bg-background-lighter border shadow-lg z-50 py-1 text-text"
        :style="{ top: `${menuY}px`, left: `${menuX}px` }">
        <div class="px-4 py-2 hover:bg-background-lighter-hover cursor-pointer"
            @click.stop="handleAction('open-folder')">
            Open in file explorer
        </div>
        <div class="px-4 py-2 hover:bg-background-lighter-hover cursor-pointer" @click.stop="handleAction('copy-path')">
            Copy Path
        </div>
    </div>
</template>
