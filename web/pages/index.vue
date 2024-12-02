<script setup lang="ts">
import {get_programs, get_render_context, init, tick} from "chip-8";
import {useEmulator} from "~/composables/useEmulator";

let programs: string[] = get_programs()
let selectedProgram = ref(programs[0])

watch(selectedProgram, () => {
  reset()
})

const {renderContext, reset, step, toggleRun, interval} = useEmulator(selectedProgram)
</script>

<template>
  <div class="flex justify-center">

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
      <div>
        <Display :render-context="renderContext"/>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>