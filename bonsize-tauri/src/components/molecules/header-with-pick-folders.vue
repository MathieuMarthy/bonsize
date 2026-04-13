<script setup lang="ts">
import PickFolderButtons from "./pick-folder-buttons.vue";

function goToHome() {
    window.location.hash = "";
}
</script>

<template>
    <div class="fixed flex justify-between items-center w-full px-4 py-2 bg-background">
        <div class="flex gap-4 items-center cursor-pointer" @click="goToHome">
            <img class="w-16 h-auto" src="../../assets/icon.png" alt="bonsize's logo" />
            <p class="text-text text-center text-2xl font-bold" href="/">Bonsize</p>
        </div>

        <PickFolderButtons :vertical="false" />
    </div>
</template>
