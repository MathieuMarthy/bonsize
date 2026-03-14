<script setup lang="ts">
import MultiButton from "./components/atoms/multi-button.vue";

</script>

<template>
    <main class="w-full h-screen flex flex-col justify-around bg-background">
        <div class="flex flex-col items-center">
            <img class="w-46 h-auto" src="./assets/icon.png" alt="bonsize's logo" />
            <p class="text-text text-center text-6xl font-bold">Bonsize</p>
        </div>

        <div class="flex flex-col items-center gap-4">
            <MultiButton class="w-64" text="Pick folder"/>
            <MultiButton class="w-64" text="Scan all my files" :full="false"/>
        </div>
    </main>
</template>
