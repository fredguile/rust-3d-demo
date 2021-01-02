const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext("webgl", { antialias: true });

import("./pkg/index")
  .then((m) => {
    if (!gl) {
      alert("Failed to initialize WeGL!");
      return;
    }

    const FPS_THROTTLE = 1000.0 / 60.0; // ms / frames
    const rustClient = new m.RustClient();
    const initialTime = Date.now();
    let lastDrawTime = -1;

    function render() {
      window.requestAnimationFrame(render);
      const currTime = Date.now();

      if (currTime < lastDrawTime + FPS_THROTTLE) {
        return;
      }

      lastDrawTime = currTime;

      if (
        window.innerHeight !== canvas.height ||
        window.innerWidth !== canvas.width
      ) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
      }

      const elapsedTime = currTime - initialTime;
      rustClient.update(elapsedTime, window.innerWidth, window.innerHeight);
      rustClient.render();
    }

    render();
  })
  .catch(console.error);
