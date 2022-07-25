<script setup lang="ts">
import { markRaw, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import MainLayout from './@core/layouts/MainLayout.vue';


const route = useRoute()
const layout = ref();


watch(
    () => route.meta.layout as string | undefined,
    async (metaLayout) => {
        try {
            const component = metaLayout && await import(`./@core/layouts/${metaLayout}.vue`)
            layout.value = markRaw(component.default || MainLayout)
        } catch(e) {
            layout.value = markRaw(MainLayout);
        }
    },
    { immediate: true }
)

</script>

<template>
    <div>
        <component :is="layout">
            <RouterView />
        </component>
    </div>
</template>

<style scoped>

</style>
