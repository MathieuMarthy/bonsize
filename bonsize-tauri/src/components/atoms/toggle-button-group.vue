<script setup lang="ts">
import { PropType, ref } from "vue";

const props = defineProps({
    values: { type: Array as PropType<string[]>, required: true },
    initIndex: { type: Number, required: false, default: 0 },
});

const selectedIndex = ref(props.initIndex >= 0 ? props.initIndex : 0);

const emit = defineEmits(["changeValue"]);

function onButtonClick(index: number) {
    selectedIndex.value = index;
    emit("changeValue", props.values[index]);
}
</script>

<template>
    <div class="flex p-1 gap-1 rounded-3xl bg-gray-800 w-fit">
        <button v-for="(value, index) in values" :key="value" class="text-center py-1 px-2 cursor-pointer"
            :class="[selectedIndex == index ? 'bg-primary rounded-2xl' : '']" @click="onButtonClick(index)">
            <p class="select-none text-text">{{ value }}</p>
        </button>
    </div>
</template>
