<script setup lang="ts">
import { PropType } from "vue";
import { type FileModel } from "../../../models/FileModel";
import { formatFileSize } from "../../../utils/format";
import { useContextMenu } from "../context-menu/useContextMenu";

const props = defineProps({
    file: { type: Object as PropType<FileModel>, required: true },
    parentPath: { type: String, default: "" },
});

const { openContextMenu } = useContextMenu();

function toggleFileOpen() {
    // eslint-disable-next-line vue/no-mutating-props
    props.file.folder_open = !props.file.folder_open;
}

function onContextMenu(event: MouseEvent) {
    openContextMenu(event, props.file);
}
</script>

<template>
    <p class="text-text text-xl cursor-pointer" :style="{ paddingLeft: `${file.depth * 20}px` }" @click="toggleFileOpen"
        @contextmenu.prevent.stop="onContextMenu">
        <span class="select-none">{{ file.is_directory ? (file.folder_open ? '📂' : '📁') : '📄' }} - {{
            formatFileSize(file.size) }} -
        </span>{{ file.path.replace(parentPath, "") }}
    </p>

    <template v-if="file.folder_open">
        <FilesList v-for="child in file.children" :key="child.path" :file="child" :parent-path="file.path + '/'" />
    </template>
</template>
