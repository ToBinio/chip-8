import {get_render_context, init, on_key_down, on_key_up, tick} from "chip-8";
import {onKeyDown, onKeyUp} from "@vueuse/core";

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

    onKeyDown(keys, (event) => {
        on_key_down(event.key)
    })

    onKeyUp(keys, (event) => {
        on_key_up(event.key)
    })

    return {renderContext, reset, step, toggleRun, interval}
}