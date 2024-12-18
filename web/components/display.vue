<script setup lang="ts">
const props = defineProps<{ renderContext: RenderContext | undefined }>()

let canvasRef = useTemplateRef("canvas");
watch(() => props.renderContext, (renderContext) => {
  if (!renderContext || !canvasRef.value)
    return

  let canvas = canvasRef.value;

  if (!canvas)
    return;

  let ctx = canvas.getContext("2d")!;
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  for (let y = 0; y < 64; y++) {
    for (let x = 0; x < 128; x++) {

      ctx.fillStyle = "black"

      if (renderContext.pixels[y * 128 + x]) {
        ctx.fillRect(x * 5, y * 5, 5, 5)
      }
    }
  }
})
</script>

<template>
  <div class="flex">
    <div>
      <div class="flex justify-between gap-3" v-for="(registry, index) in renderContext?.registries">
        <div class="text-sm">V{{ index }}</div>
        <div> 0x{{ registry.toString(16) }}</div>
      </div>
    </div>
    <canvas width="640" height="320" class="border-4 border-black" ref="canvas"/>
  </div>
</template>

<style scoped>

</style>