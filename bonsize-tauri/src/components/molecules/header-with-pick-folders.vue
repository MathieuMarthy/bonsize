<script setup lang="ts">
import PickFolderButtons from "./pick-folder-buttons.vue";

</script>

<template>
    <div class="flex justify-between items-center px-4 py-2">
        <div class="flex gap-4 items-center">
            <img class="w-16 h-auto" src="../../assets/icon.png" alt="bonsize's logo" />
            <a class="text-text text-center text-2xl font-bold" href="/">Bonsize</a>
        </div>

        <PickFolderButtons :vertical="false" />
    </div>
</template>
