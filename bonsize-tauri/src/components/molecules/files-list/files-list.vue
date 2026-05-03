<script setup lang="ts">
import { PropType } from "vue";
import { FileModel } from "../../../models/FileModel";
import FileRepresentation from "../../atoms/file-representation.vue";

defineProps({
    files: { type: Array as PropType<FileModel[]>, required: true },
});
</script>

<template>
    <FileRepresentation v-for="file in files" :key="file.path" :file="file" />
</template>
