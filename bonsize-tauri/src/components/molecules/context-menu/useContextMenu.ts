import { ref } from "vue";
import type { FileModel } from "../../../models/FileModel";

const showContextMenu = ref(false);
const menuX = ref(0);
const menuY = ref(0);
const currentFile = ref<FileModel | null>(null);
const parentFile = ref<FileModel | null>(null);

export function useContextMenu() {
    function openContextMenu(event: MouseEvent, file: FileModel, parent: FileModel | null = null) {
        showContextMenu.value = true;
        menuX.value = event.clientX;
        menuY.value = event.clientY;
        currentFile.value = file;
        parentFile.value = parent;
    }

    function closeContextMenu() {
        showContextMenu.value = false;
        currentFile.value = null;
        parentFile.value = null;
    }

    return {
        showContextMenu,
        menuX,
        menuY,
        currentFile,
        parentFile,
        openContextMenu,
        closeContextMenu
    };
}
