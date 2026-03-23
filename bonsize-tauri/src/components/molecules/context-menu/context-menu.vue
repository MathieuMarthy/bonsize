<script setup lang="ts">
import { openPath } from "@tauri-apps/plugin-opener";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useContextMenu } from "./useContextMenu";
import { invoke } from "@tauri-apps/api/core";
import { useNotification } from "../../../utils/useNotification";
import ConfirmModal from "../../atoms/confirm-modal.vue";

const { showContextMenu, menuX, menuY, currentFile, parentFile, closeContextMenu } = useContextMenu();
const { notify } = useNotification();
const confirmModalRef = ref<InstanceType<typeof ConfirmModal> | null>(null);

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

    const type = currentFile.value.is_directory ? "folder" : "file";
    const confirmed = await confirmModalRef.value?.show(`Are you sure you want to delete this ${type}?`);
    if (!confirmed) {
        return;
    }

    try {
        const fileIsDeleted = await invoke("delete_file", { path: currentFile.value.path });
        if (fileIsDeleted) {
            if (parentFile.value !== null) {
                parentFile.value.children = parentFile.value.children
                    .filter((child) => child.path !== currentFile.value!.path);
            }
            notify(`${type.charAt(0).toUpperCase() + type.slice(1)} deleted successfully`, "success");
        } else {
            notify(`Failed to delete ${type}`, "error");
        }
    } catch {
        notify(`Error deleting ${type}`, "error");
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
    <ConfirmModal ref="confirmModalRef" />
</template>
