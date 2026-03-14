<script setup lang="ts">
import { PropType } from "vue";
import { type FileModel } from "../../../models/FileModel";

const props = defineProps({
    file: { type: Object as PropType<FileModel>, required: true },
    parentPath: { type: String, default: "/" },
});

function toggleFileOpen() {
    // eslint-disable-next-line vue/no-mutating-props
    props.file.folder_open = !props.file.folder_open;
}
</script>

<template>
    <p class="text-text text-xl cursor-pointer" :style="{ paddingLeft: `${file.depth * 10}px` }"
        @click="toggleFileOpen">
        {{ file.is_directory ? (file.folder_open ? '📂' : '📁') : '📄' }} - {{ file.size }} - {{
            file.path.replace(parentPath, "") }}
    </p>

    <template v-if="file.folder_open">
        <FilesList v-for="child in file.children" :key="child.path" :file="child" :parent-path="file.path" />
    </template>
</template>
