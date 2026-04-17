<script setup lang="ts">
import { ref, onMounted, Ref } from "vue";
import { FileModel } from "../../models/FileModel";
import { invoke } from "@tauri-apps/api/core";
import HeaderWithPickFolders from "../molecules/header-with-pick-folders.vue";
import LoaderFilesList from "../molecules/files-list/loader-files-list.vue";
import FilesList from "../molecules/files-list/files-list.vue";
import { attachParents } from "../../utils/updateFileTree";

const isLoading = ref(true);
const directoryInformations: Ref<FileModel | undefined> = ref(undefined);

onMounted(() => {
    const hash = window.location.hash;
    const urlParams = new URLSearchParams(hash.split("?")[1] || "");
    const folderPath = urlParams.get("path") || "";

    get_directory_informations(folderPath);
});

async function get_directory_informations(path: string) {
    const data: FileModel | undefined = await invoke("scan_directory", { path: path });

    if (data === undefined) {
        return;
    }

    attachParents(data);

    directoryInformations.value = data;
    directoryInformations.value.folder_open = true;
    isLoading.value = false;
}
</script>

<template>
    <div>
        <HeaderWithPickFolders />

        <div class="flex pt-32">
            <div class="pl-16 pt-12 w-full">
                <LoaderFilesList v-if="isLoading" />
                <FilesList :file="directoryInformations!" v-else />
            </div>
        </div>
    </div>
</template>
