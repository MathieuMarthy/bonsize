<script setup lang="ts">
import PickFolderButtons from "../molecules/pick-folder-buttons.vue";
</script>

<template>
    <main class="w-full h-screen flex flex-col justify-around">
        <div class="flex flex-col items-center">
            <img class="w-46 h-auto" src="../../assets/icon.png" alt="bonsize's logo" />
            <p class="text-text text-center text-6xl font-bold">Bonsize</p>
        </div>

        <PickFolderButtons/>
    </main>
</template>
