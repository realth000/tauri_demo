<script lang="ts" setup>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const myCommandArg = ref("");
const myCommandResult = ref("");

const clickCount = ref(0);

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", {name: name.value});
}

async function myCommand() {
  const time = new Date().toString();
  myCommandResult.value = await invoke("my_command", {name: myCommandArg.value, time: time});
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>

  <br>

  <form class="row" @submit.prevent="myCommand">
    <input id="my-command-input" v-model="myCommandArg" placeholder="Run my command..."/>
    <button type="submit">Run My Command</button>
  </form>

  <p> {{ myCommandResult }}</p>

  <p>{{ clickCount }}
    <button @click="clickCount++">Add</button>
  </p>

</template>
