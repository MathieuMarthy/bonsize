<template>
    <div class="flex flex-col animate-pulse gap-2 w-4/12">
        <div v-for="i in Array(30).keys()" :key="i" class="h-3 rounded bg-gray-800 w-full" />
    </div>
</template>
