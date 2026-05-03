<script setup lang="ts">
import { PropType } from "vue";
import { FileModel } from "../../models/FileModel";
import { useContextMenu } from "../molecules/context-menu/useContextMenu";
import { formatFileSize } from "../../utils/format";

const props = defineProps({
    file: { type: Object as PropType<FileModel>, required: true },
    depth: { type: Number, default: 0 },
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
    <p class="text-text text-xl cursor-pointer select-none" :style="{ paddingLeft: `${depth * 20}px` }"
        @click="toggleFileOpen" @contextmenu.prevent.stop="onContextMenu">
        {{ file.is_directory ? (file.folder_open ? '📂' : '📁') : '📄' }} -
        {{ formatFileSize(file.size) }} -
        <span class="has-tooltip relative">
            {{ file.path.replace(file.parent?.path ?? "", "") }}
            <span class="tooltip rounded-xl shadow-lg p-1 bg-gray-800 text-text select-text ml-14">
                {{ file.path }}
            </span>
        </span>
    </p>

    <template v-if="file.folder_open">
        <FileRepresentation v-for="child in file.children" :key="child.path" :file="child" :depth="depth + 1" />
    </template>
</template>
