import { clearScreenToColor, drawRectangle } from "./usecases/index.js"

export let imports = {
    env: {
        js_clear_screen_to_color: clearScreenToColor,
        js_draw_rectangle: drawRectangle
    }
};