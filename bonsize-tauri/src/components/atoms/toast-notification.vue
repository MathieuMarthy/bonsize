<script setup lang="ts">
import { useNotification } from "../../utils/useNotification";

const { notifications, removeNotification } = useNotification();
</script>

<template>
    <div class="fixed bottom-4 right-4 z-100 flex flex-col gap-2">
        <div v-for="notification in notifications" :key="notification.id"
            class="px-4 py-3 rounded shadow-lg text-white max-w-sm transition-all" :class="{
                'bg-green-500': notification.type === 'success',
                'bg-red-500': notification.type === 'error',
                'bg-blue-500': notification.type === 'info'
            }" @click="removeNotification(notification.id)">
            <p>{{ notification.message }}</p>
        </div>
    </div>
</template>
