<script setup lang="ts">
import SidebarComponent from '../components/Sidebar/SidebarComponent.vue';
</script>

<template>
    <div class="flex">
        <div class="flex flex-col justify-between w-16 h-screen bg-white border-r">
            <SidebarComponent />
        </div>
        <!-- <div class="flex flex-col justify-between w-16 h-screen bg-white border-r">
        </div> -->
            <slot></slot>
    </div>
</template>