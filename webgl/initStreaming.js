import { imports } from "./imports.js";

WebAssembly.instantiateStreaming(fetch('target/wasm32-unknown-unknown/debug/examples/game.wasm'), imports).then(function (result) {
    result.instance.exports.main();

    document.onkeydown = function (event) {
        let code = 0;
        switch (event.code) {
            case "ArrowLeft":
                code = 1;
                break;
            case "ArrowRight":
                code = 2;
                break;
            case "ArrowUp":
                code = 3;
                break;
            case "ArrowDown":
                code = 4;
                break;
            case "Space":
                code = 5;
                break;
        }

        result.instance.exports.key_pressed(code);

        function animate() {
            result.instance.exports.animate();
            requestAnimationFrame(animate);
        }
        requestAnimationFrame(animate);
    };
})