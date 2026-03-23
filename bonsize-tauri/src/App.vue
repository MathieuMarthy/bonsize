<script setup lang="ts">
import { computed, ref, type Component } from "vue";
import FolderDetails from "./components/pages/folder-details.vue";
import HomePage from "./components/pages/home-page.vue";
import ContextMenu from "./components/molecules/context-menu/context-menu.vue";
import ToastNotification from "./components/atoms/toast-notification.vue";

const routes: { [id: string]: Component } = {
    "/": HomePage,
    "/folder-details": FolderDetails,
};

const currentPath = ref(window.location.hash);

window.addEventListener("hashchange", () => {
    currentPath.value = window.location.hash;
});

const currentView = computed(() => {
    const rawPath = currentPath.value.split("?")[0] || "#/";
    const currentPathSliced: string = rawPath.slice(1);
    return routes[currentPathSliced] || HomePage;
});
</script>

<template>
    <component :is="currentView" />
    <ContextMenu />
    <ToastNotification />
</template>
