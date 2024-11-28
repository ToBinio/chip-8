<script setup lang="ts">
import {get_programs, get_render_context, init, tick} from "chip-8";

export type RenderContext = {
  title: string,
  registries: [number],
  pixels: [boolean],
}

let programs: string[] = get_programs()
let selectedProgram = ref(programs[0])

watch(selectedProgram, () => {
  reset()
})

onMounted(() => {
  reset()
})

function reset() {
  init(selectedProgram.value)
  step()
}

let renderContext = ref<RenderContext | undefined>(undefined);

function step() {
  tick()
  renderContext.value = get_render_context()
  render()
}


let interval = ref<number | undefined>(undefined);

function toggleRun() {

  if (interval.value) {
    clearInterval(interval.value)
    interval.value = undefined;
    return
  }

  interval.value = setInterval(() => {
    step();
  }, 0) as unknown as number
}

let canvasRef = useTemplateRef("canvas");

function render() {
  if (!renderContext.value || !canvasRef.value)
    return

  let canvas = canvasRef.value;

  if (!canvas)
    return;

  let ctx = canvas.getContext("2d")!;
  ctx.clearRect(0, 0, canvas.width, canvas.height)

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
  <div class="flex justify-center">
    <div>
      <div class="flex justify-between gap-3" v-for="(registry, index) in renderContext?.registries">
        <div class="text-sm">V{{ index }}</div>
        <div> 0x{{ registry.toString(16) }}</div>
      </div>
    </div>
    <div class="flex flex-col gap-2 items-center">
      <div class="flex gap-5">
        <button class="bg-gray-200 p-2 hover:bg-gray-300" @click="step">
          step
        </button>
        <button class="bg-gray-200 p-2 hover:bg-gray-300" :class="{'rounded-full': interval != undefined}"
                @click="toggleRun">
          run
        </button>
        <button class="bg-gray-200 p-2 hover:bg-gray-300" @click="reset">
          reset
        </button>
        <select v-model="selectedProgram">
          <option v-for="option in programs" :value="option">
            {{ option }}
          </option>
        </select>
      </div>
      <canvas width="640" height="320" class="border-4 border-black" ref="canvas"/>
    </div>
  </div>
</template>

<style scoped>

</style>