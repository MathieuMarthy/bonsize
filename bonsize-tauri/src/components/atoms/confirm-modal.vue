<script setup lang="ts">
import { ref } from "vue";

const isVisible = ref(false);
const message = ref("");
let resolvePromise: ((value: boolean) => void) | null = null;

function show(msg: string): Promise<boolean> {
    message.value = msg;
    isVisible.value = true;
    return new Promise((resolve) => {
        resolvePromise = resolve;
    });
}

function confirm() {
    isVisible.value = false;
    if (resolvePromise) {
        resolvePromise(true);
    }
}

function cancel() {
    isVisible.value = false;
    if (resolvePromise) {
        resolvePromise(false);
    }
}

defineExpose({ show });
</script>

<template>
    <div v-if="isVisible" class="fixed inset-0 z-200 flex items-center justify-center bg-black bg-opacity-50">
        <div class="bg-background border border-background-lighter shadow-xl rounded-lg p-6 max-w-xl w-full text-text">
            <p class="text-lg mb-6">{{ message }}</p>
            <div class="flex justify-end gap-4">
                <button @click="cancel" class="px-4 py-2 rounded bg-background-lighter hover:bg-background-lighter-hover
                        transition-colors cursor-pointer">
                    Cancel
                </button>
                <button @click="confirm"
                    class="px-4 py-2 rounded bg-red-600 hover:bg-red-700 text-white transition-colors cursor-pointer">
                    Confirm
                </button>
            </div>
        </div>
    </div>
</template>
