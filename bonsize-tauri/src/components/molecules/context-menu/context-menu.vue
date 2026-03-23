<script setup lang="ts">
import { openPath } from "@tauri-apps/plugin-opener";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { computed, onMounted, onUnmounted } from "vue";
import { useContextMenu } from "./useContextMenu";
import { invoke } from "@tauri-apps/api/core";
import { useNotification } from "../../../utils/useNotification";

const { showContextMenu, menuX, menuY, currentFile, parentFile, closeContextMenu } = useContextMenu();
const { notify } = useNotification();
const actions = computed(() => [
    ["Open in file explorer", "open-folder"],
    ["Copy Path", "copy-path"],
    [`Delete ${currentFile.value?.is_directory ? "folder" : "file"}`, "delete-file"],
]);

async function handleAction(action: string) {
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

        case "delete-file":
            await deleteFile();
            break;
        default:
            break;
    }
    closeContextMenu();
}

async function deleteFile() {
    if (currentFile.value === null) {
        return;
    }

    try {
        const fileIsDeleted = await invoke("delete_file", { path: currentFile.value.path });
        if (fileIsDeleted) {
            if (parentFile.value !== null) {
                parentFile.value.children = parentFile.value.children
                    .filter((child) => child.path !== currentFile.value!.path);
            }
            notify(`${currentFile.value.is_directory ? "Folder" : "File"} deleted successfully`, "success");
        } else {
            notify(`Failed to delete ${currentFile.value.is_directory ? "folder" : "file"}`, "error");
        }
    } catch {
        notify(`Error deleting ${currentFile.value.is_directory ? "folder" : "file"}`, "error");
    }
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
        <div v-for="action in actions" :key="action[1]"
            class="px-4 py-2 hover:bg-background-lighter-hover cursor-pointer" @click.stop="handleAction(action[1])">
            <p>{{ action[0] }}</p>
        </div>
    </div>
</template>
