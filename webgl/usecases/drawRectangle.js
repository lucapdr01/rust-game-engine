import { gl, program, position_attribute_location, color_uniform_location } from "../webgl.js";

export function drawRectangle (x, y, width, height, red, green, blue, alpha) {
    let data_buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, data_buffer);

    function adjust_pos(size, pos) {
        return (pos / size) * 2.0 - 1.0;
    }
    gl.bufferData(
        gl.ARRAY_BUFFER,
        new Float32Array([
            adjust_pos(gl.canvas.width, x), adjust_pos(gl.canvas.height, y),
            adjust_pos(gl.canvas.width, x + width), adjust_pos(gl.canvas.height, y),
            adjust_pos(gl.canvas.width, x + width), adjust_pos(gl.canvas.height, y + height),
            adjust_pos(gl.canvas.width, x), adjust_pos(gl.canvas.height, y + height)
        ]),
        gl.STATIC_DRAW);

    gl.vertexAttribPointer(
        position_attribute_location,
        2,          // How many numbers are in each value of our data. In our case it's 2 because we're passing in 2D coordinates as vec2.
        gl.FLOAT,   // What type of numbers are used our data
        false, 0, 0 // These aren't important to understand now.
    );

    gl.useProgram(program);
    gl.uniform4f(color_uniform_location, red, green, blue, alpha);
    gl.drawArrays(gl.TRIANGLE_FAN, 0, 4);
    gl.deleteBuffer(data_buffer);
}