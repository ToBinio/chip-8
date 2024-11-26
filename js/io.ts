export type RenderContext = {
    title: string,
    registries: [number],
    pixels: [boolean],
}

export function render(context: RenderContext) {
    console.log(context);
}