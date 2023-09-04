import { gl } from "../webgl.js";

export function clearScreenToColor(red, green, blue, alpha) {
  gl.clearColor(red, green, blue, alpha);
  gl.clear(gl.COLOR_BUFFER_BIT);
}
