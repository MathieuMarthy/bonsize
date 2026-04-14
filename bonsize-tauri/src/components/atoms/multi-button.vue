<script setup lang="ts">
defineProps({
    text: { type: String, required: true },
    full: { type: Boolean, default: true },
});
</script>

<template>
    <button class="rounded-2xl px-6 py-1 text-text transition-colors duration-200 hover:cursor-pointer
            text-xl" :class="[
                full ?
                    'bg-primary hover:bg-primary-hover' :
                    'border-primary hover:border-primary-hover hover:bg-black border-2'
            ]">{{ text }}</button>
</template>
