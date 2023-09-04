let canvas = document.getElementById("engine_canvas");
let gl = canvas.getContext("webgl");

let vertex_shader = gl.createShader(gl.VERTEX_SHADER);

gl.shaderSource(
  vertex_shader,
  `
    attribute vec2 vertex_position;
    void main(void) {
        gl_Position = vec4(vertex_position, 0.0, 1.0);
    }
`
);

gl.compileShader(vertex_shader);

let fragment_shader = gl.createShader(gl.FRAGMENT_SHADER);

gl.shaderSource(
  fragment_shader,
  `
    precision mediump float;
    uniform vec4 color;
    void main() {
        gl_FragColor = color;
    }
`
);

gl.compileShader(fragment_shader);

let program = gl.createProgram();

gl.attachShader(program, vertex_shader);
gl.attachShader(program, fragment_shader);
gl.linkProgram(program);

// We'll need to know this "location" later to let WebGL know where our rectangle corner data should go.
let position_attribute_location = gl.getAttribLocation(
  program,
  "vertex_position"
);

gl.enableVertexAttribArray(position_attribute_location);
let color_uniform_location = gl.getUniformLocation(program, "color");

export { gl, program, position_attribute_location, color_uniform_location}
