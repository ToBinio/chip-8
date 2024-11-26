<script setup lang="ts">
import {get_render_context, init, tick} from "chip-8";

export type RenderContext = {
  title: string,
  registries: [number],
  pixels: [boolean],
}

onMounted(() => {
  init()

  step()
})

let renderContext = ref<RenderContext>(undefined);


function step() {
  tick()
  renderContext.value = get_render_context()
  render()
}

let canvasRef = useTemplateRef("canvas");

function render() {
  let canvas = canvasRef.value!;
  let ctx = canvas.getContext("2d")!;

  ctx.clearRect(0, 0, 500, 500)

  for (let y = 0; y < 32; y++) {
    for (let x = 0; x < 64; x++) {

      ctx.fillStyle = "black"

      if (renderContext.value.pixels[y * 64 + x]) {
        ctx.fillRect(x * 10, y * 10, 10, 10)
      }
    }
  }
}

</script>

<template>
  <div class="flex flex-col items-center">
    <button class="bg-gray-200 p-2 hover:bg-gray-300" @click="step">
      step
    </button>
    <canvas width="640" height="320" class="border-4 border-black" ref="canvas"/>
  </div>
</template>

<style scoped>

</style>