import {get_render_context, init, tick} from "chip-8";
import {onKeyDown} from "@vueuse/core";

export type RenderContext = {
    title: string,
    registries: [number],
    pixels: [boolean],
}

export function useEmulator(selectedProgram: Ref<string>) {
    onMounted(() => {
        reset()
    })

    function reset() {
        stopTicking()
        init(selectedProgram.value)
        step()
    }

    let renderContext = ref<RenderContext | undefined>(undefined);

    function step() {
        tick()
        renderContext.value = get_render_context()
    }


    let interval = ref<number | undefined>(undefined);

    function toggleRun() {

        if (interval.value) {
            stopTicking()
            return
        }

        interval.value = setInterval(() => {
            step();
        }, 0) as unknown as number
    }

    function stopTicking() {
        clearInterval(interval.value)
        interval.value = undefined;
    }

    return {renderContext, reset, step, toggleRun, interval}
}