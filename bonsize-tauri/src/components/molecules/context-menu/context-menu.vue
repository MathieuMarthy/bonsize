<script setup lang="ts">
import { openPath } from "@tauri-apps/plugin-opener";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useContextMenu } from "./useContextMenu";
import { invoke } from "@tauri-apps/api/core";
import { useNotification } from "../../../utils/useNotification";
import ConfirmModal from "../../atoms/confirm-modal.vue";
import { attachParents, updateParentSizes } from "../../../utils/updateFileTree";
import { FileModel } from "../../../models/FileModel";

const { showContextMenu, menuX, menuY, currentFile, parentFile, closeContextMenu } = useContextMenu();
const { notify } = useNotification();
const confirmModalRef = ref<InstanceType<typeof ConfirmModal> | null>(null);

const actions = computed(() => [
    ["Open in file explorer", "open-folder"],
    ["Copy Path", "copy-path"],
    ["Refresh branch", "refresh-branch"],
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
            notify("Path copied to clipboard", "success");
            break;

        case "refresh-branch":
            rescanFile();
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
    const fileToDelete = currentFile.value;
    const parent = parentFile.value;

    if (fileToDelete === null) {
        return;
    }

    const type = fileToDelete.is_directory ? "folder" : "file";
    const confirmed = await confirmModalRef.value?.show(`Are you sure you want to delete ${fileToDelete.path}?`);
    if (!confirmed) {
        return;
    }

    try {
        const fileIsDeleted = await invoke("delete_file", { path: fileToDelete.path });

        if (fileIsDeleted) {
            const deletedSize = fileToDelete.size;

            if (parent !== null) {
                parent.children = parent.children
                    .filter((child) => child.path !== fileToDelete.path);
                updateParentSizes(parent, -deletedSize);
            }

            notify(`${type.charAt(0).toUpperCase() + type.slice(1)} deleted successfully`, "success");
        } else {
            notify(`Failed to delete ${type}`, "error");
        }
    } catch {
        notify(`Error deleting ${type}`, "error");
    }
}

async function rescanFile() {
    const file = currentFile.value;
    const parent = parentFile.value;

    if (file === null) {
        return;
    }

    const updatedFile: FileModel | undefined = await invoke("scan_directory", { path: file.path });

    if (updatedFile === undefined) {
        return;
    }

    const sizeDiff = updatedFile.size - file.size;
    const oldFolderOpen = file.folder_open;

    Object.assign(file, updatedFile);
    file.folder_open = oldFolderOpen;

    attachParents(file, parent ?? undefined);

    if (parent !== null) {
        // Sort directories before files, then sort by descending size.
        parent.children.sort((a, b) =>
            ((b.is_directory ? 1 : 0) - (a.is_directory ? 1 : 0)) ||
            (b.size - a.size),
        );
        updateParentSizes(parent, sizeDiff);
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
