<script setup lang="ts">
import { ref, onMounted, Ref } from "vue";
import { FileModel } from "../../models/FileModel";
import { invoke } from "@tauri-apps/api/core";
import HeaderWithPickFolders from "../molecules/header-with-pick-folders.vue";
import LoaderFilesList from "../molecules/files-list/loader-files-list.vue";
import FilesList from "../molecules/files-list/files-list.vue";
import { attachParents, flattenFiles } from "../../utils/updateFileTree";
import { DELAY_BEFORE_SEARCH } from "../../const";
import SnipperLoader from "../atoms/snipper-loader.vue";

const isLoading = ref(true);
const rootFile: Ref<FileModel | undefined> = ref(undefined);

const searchResults: Ref<FileModel[]> = ref([]);
const flatFiles: Ref<FileModel[]> = ref([]);
const searchQuery = ref("");
const searchIsLoading = ref(false);
let searchTimeout: ReturnType<typeof setTimeout> | null = null;

function onSearchInput(event: Event) {
    const target = event.target as HTMLInputElement;
    searchQuery.value = target.value;
    searchIsLoading.value = true;

    if (searchTimeout) {
        clearTimeout(searchTimeout);
    }

    searchTimeout = setTimeout(() => {
        const query = searchQuery.value.toLowerCase();
        const results = [];

        for (const f of flatFiles.value) {
            const pathSlashIdx = f.path.lastIndexOf("/");
            const filename = pathSlashIdx !== -1 ? f.path.slice(pathSlashIdx + 1) : f.path;

            if (filename.toLowerCase().includes(query)) {
                results.push(f);
                if (results.length >= 100) {
                    break;
                }
            }
        }

        searchResults.value = results;
        searchIsLoading.value = false;
    }, DELAY_BEFORE_SEARCH);
}

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

    rootFile.value = data;
    rootFile.value.folder_open = true;

    flatFiles.value = flattenFiles(rootFile.value);

    isLoading.value = false;
}
</script>

<template>
    <div>
        <HeaderWithPickFolders />

        <div class="flex pt-24">
            <div class="flex flex-col gap-4 pl-16 pt-12 w-full">
                <div class="w-96 flex items-center gap-4">
                    <input class="py-2 px-5 w-11/12 bg-background-lighter text-text rounded-3xl"
                        placeholder="Search files" @input="onSearchInput" />
                    <SnipperLoader v-if="searchIsLoading" />
                </div>

                <div>
                    <LoaderFilesList v-if="isLoading" />
                    <FilesList :files="searchQuery === '' ? [rootFile!] : searchResults" v-else />
                </div>
            </div>
        </div>
    </div>
</template>
