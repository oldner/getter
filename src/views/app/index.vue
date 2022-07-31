<script setup lang="ts">
import { onMounted, Ref, ref } from 'vue';

import { invoke } from '@tauri-apps/api'
import LoadingComponent from '../../@core/components/Sidebar/Loading/LoadingComponent.vue'
import JsonViewer from 'vue-json-viewer'

const response: Record<string, any> = ref(null);
const url: Ref<string> = ref('');
const isLoading: Ref<boolean> = ref(false);

async function request() {
  if(url.value !== '') {
    isLoading.value = true;
    await invoke("greet", {
      request: {
        url: url.value,
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        },
        bodyType: 'json',
        body: ''
      }
    })
    .then(res => {
      response.value = res;
      isLoading.value = false;
    })
    .catch(err => {
      response.value = err;
      isLoading.value = false;
    });
  }
}


</script>

<template>
<div class="w-full container mx-auto">
  <div class="flex items-center mt-24">
    <input v-model="url" class="mr-4 shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Type your url">
    <button type="button" @click="request" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">
      <span v-if="!isLoading">Send</span>
      <LoadingComponent v-else />
    </button>
  </div>
  <div class="mt-4" v-if="response && !isLoading">
    <h4>Response</h4>
    <json-viewer
      :value="response"
      :expand-depth=5
      copyable
      boxed
      sort></json-viewer>
  </div>
  <div v-if="isLoading">
    <LoadingComponent />
  </div>
</div>
</template>