const rust = import('./pkg/runner_game');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', { antialias: true }); // antialias smooths things - comes at performance cost

rust.then(m => {
    if (!gl) {
        alert('Failed to initialize WebGL');
        return;
    }

    //gl.enable(gl.BLEND); // supports semi transparent objects - fading in and out smoothly - comes at large performance cost - messes with optimized depth buffering as well
    //gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA); // smooth blending function

    const FPS_THROTTLE = 1000.0 / 60.0; // milliseconds / frames - setting as a const so that it doesn't waste performance going as fast as possible
    const client = m.Client.new();
    const initialTime = Date.now();
    var lastDrawTime = -1; // milliseconds

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currTime;

            // readjust for browser resizing
            if (window.innerHeight != canvas.height || window.innerWidth != canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currTime - initialTime;
            client.update(elapsedTime, window.innerHeight, window.innerWidth);
            client.render();
        }
    }

    render();
});