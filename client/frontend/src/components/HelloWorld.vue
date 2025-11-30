<script setup>
import { onMounted } from "vue";

onMounted(async () => {
    // dynamic import avoids eval
    const wasm = await import("../pkg/client_wasm.js");

    await wasm.default(); // initialize WASM
    wasm.start_render(); // start Bevy
});
</script>

<template>
    <canvas id="game"></canvas>
</template>
