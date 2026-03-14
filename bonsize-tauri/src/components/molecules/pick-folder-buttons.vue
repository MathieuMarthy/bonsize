<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";

import MultiButton from "../atoms/multi-button.vue";

defineProps({
    vertical: { type: Boolean, default: true },
});

async function pick_folder() {
    const folder = await open({
        multiple: false,
        directory: true,
    });

    if (folder === null) {
        return;
    }

    window.location.hash = `#/folder-details?path=${folder}`;
}
</script>

<template>
    <div class="flex gap-4" :class="[
        vertical ?
            'flex-col items-center' :
            'flex flex-row-reverse'
    ]">
        <MultiButton :class="[vertical ? 'w-64 py-3' : 'h-12']" text="Pick folder" @click="pick_folder" />
        <MultiButton :class="[vertical ? 'w-64 py-3' : 'h-12']" text="Scan all my files" :full="false" />
    </div>
</template>
