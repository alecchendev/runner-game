/*import('runner-game')
  .catch(console.error);
//*/
import { Universe, Cell, Player } from "runner-game";
import { memory } from "runner-game/runner_game_bg";
const { mat4, mat3, vec3 } = glMatrix;

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const player = Player.new();

let cameraPosition = {
  x: 0,
  y: 0,
  z: 0,
}
let cameraAngle = {
  theta: 0,
  phi: 0,
}

const universe = Universe.new();
const positions = universe.positions();
const faceColors = universe.colors();
const indices = universe.indices();

function initShaderProgram(gl, vsSource, fsSource) {
  const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
  const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

  const shaderProgram = gl.createProgram();
  gl.attachShader(shaderProgram, vertexShader);
  gl.attachShader(shaderProgram, fragmentShader);
  gl.linkProgram(shaderProgram);

  if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
    alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
    return null;
  }

  return shaderProgram;
}

function loadShader(gl, type, source) {
  const shader = gl.createShader(type);

  gl.shaderSource(shader, source);

  gl.compileShader(shader);

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    alert('An error occurred compiling the shaders ' + gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
    return null;
  }

  return shader;
}

function initBuffers(gl) {
  const positionBuffer = gl.createBuffer();

  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

  

  gl.bufferData(gl.ARRAY_BUFFER,
                new Float32Array(positions),
                gl.STATIC_DRAW);


  if (faceColors.length % 4 !== 0) {
    throw 'faceColors not divisible by 4!';
  }

  let colors = [];
  for (let j = 0; j < faceColors.length / 4; ++j) {
    const c = [];
    for (let i = 0; i < 4; ++i) {
      c.push(faceColors[4* j + i]);
    }
    colors = colors.concat(c, c, c, c);
  }

  const colorBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);

  //---

  const indexBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);

  

  gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW);

  return {
    position: positionBuffer,
    color: colorBuffer,
    indices: indexBuffer,
  };
}

function drawScene(gl, programInfo, buffers, deltaTime) {
  gl.clearColor(0.012, 0.647, 0.988, 1.0);
  gl.clearDepth(1.0);
  gl.enable(gl.DEPTH_TEST);
  gl.depthFunc(gl.LEQUAL);

  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

  const fieldOfView = 45 * Math.PI / 180;
  const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
  const zNear = 0.1;
  const zFar = 100.0;

  const projectionMatrix = mat4.create();
  mat4.perspective(projectionMatrix,
                  fieldOfView,
                  aspect,
                  zNear,
                  zFar);

  const modelViewMatrix = mat4.create();
  mat4.rotate(modelViewMatrix,
    modelViewMatrix,
    cameraAngle.phi,
    [1, 0, 0]);
  mat4.rotate(modelViewMatrix,
    modelViewMatrix,
    cameraAngle.theta,
    [0, 1, 0]);
  mat4.translate(modelViewMatrix,
    modelViewMatrix,
    [-cameraPosition.x, -cameraPosition.y, cameraPosition.z]);
  {
    const numComponents = 3;  // pull out 2 values per iteration - 2d..?
    const type = gl.FLOAT;    // the data in the buffer is 32bit floats
    const normalize = false;  // don't normalize
    const stride = 0;         // how many bytes to get from one set of values to the next
                              // 0 = use type and numComponents above
    const offset = 0;         // how many bytes inside the buffer to start from
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexPosition,
        numComponents,
        type,
        normalize,
        stride,
        offset);
    gl.enableVertexAttribArray(
        programInfo.attribLocations.vertexPosition);
  }
  {
    const numComponents = 4;
    const type = gl.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.color);
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexColor,
        numComponents,
        type,
        normalize,
        stride,
        offset);
    gl.enableVertexAttribArray(
        programInfo.attribLocations.vertexColor);
  }

  gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffers.indices);

  // Tell WebGL to use our program when drawing

  gl.useProgram(programInfo.program);

  // Set the shader uniforms

  gl.uniformMatrix4fv(
      programInfo.uniformLocations.projectionMatrix,
      false,
      projectionMatrix);
  gl.uniformMatrix4fv(
      programInfo.uniformLocations.modelViewMatrix,
      false,
      modelViewMatrix);

  {

    const vertexCount = positions.length / 2;
    const type = gl.UNSIGNED_SHORT;
    const offset = 0;
    gl.drawElements(gl.TRIANGLES, vertexCount, type, offset);
  }
}


function main() {
  const canvas = document.getElementById("runner-game-canvas");
  canvas.width = 1080;
  canvas.height = 720;

  const gl = canvas.getContext('webgl');

  if (gl === null) {
      alert("unable to initialize WebGL. Your browser or machine may not support it.");
      return;
  }

  

  // SHADERS

  // Vertex shader
  const vsSource = `
    attribute vec4 aVertexPosition;
    attribute vec4 aVertexColor;

    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;

    varying lowp vec4 vColor;

    void main() {
      gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
      vColor = aVertexColor;
    }

  `;

  // Fragment shader
  const fsSource = `
    varying lowp vec4 vColor;

    void main() {
      gl_FragColor = vColor;
    }
  `;

  const shaderProgram = initShaderProgram(gl, vsSource, fsSource);

  const programInfo = {
    program: shaderProgram,
    attribLocations: {
      vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
      vertexColor: gl.getAttribLocation(shaderProgram, 'aVertexColor'),
    },
    uniformLocations: {
      projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
      modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
    }
  };

  const buffers = initBuffers(gl);

  let then = 0;

  document.addEventListener("click", function () {
    document.body.requestPointerLock();
  });

  document.body.addEventListener("mousemove", function (event) {
    if (document.pointerLockElement === document.body) {
      //console.log("Moved by " + event.movementX + ", " + event.movementY);
      player.mouse_look(event.movementX, event.movementY);
    }
    
  });

  const LOOK = {
    "ArrowLeft": 0,
    "ArrowUp": 1,
    "ArrowRight": 2,
    "ArrowDown": 3,
  };

  const MOVE = {
    "a": 0,
    "w": 1,
    "d": 2,
    "s": 3,
    " ": 4,
  };

  document.addEventListener('keydown', function(event) {
    if (event.defaultPrevented) {
      return; // Do nothing if the event was already processed
    }

    // this is automatic in at least chrome but just in case
    if (event.key === "esc") {
      document.exitPointerLock();
    }

    if (event.key in LOOK) {
      player.look(LOOK[event.key]);
    }

    if (event.key in MOVE) {
      player.go(MOVE[event.key]);
    }

    event.preventDefault();
  });

  document.addEventListener('keyup', function(event) {
    if (event.defaultPrevented) {
      return; // Do nothing if the event was already processed
    }
  
    if (event.key in LOOK) {
      player.stop_look(LOOK[event.key]);
    }

    if (event.key in MOVE) {
      player.stop(MOVE[event.key]);
    }

    event.preventDefault();
  });

  function render(now) {
    now *= 0.001;
    const deltaTime = now - then;
    then = now;

    player.update();

    let pos = player.position();
    let theta = player.theta();
    let phi = player.phi();

    cameraPosition = {
      x: pos[0],
      y: pos[1],
      z: pos[2],
    }
    cameraAngle = {
      theta: theta,
      phi: phi,
    }

    drawScene(gl, programInfo, buffers, deltaTime);

    requestAnimationFrame(render);
  }

  requestAnimationFrame(render);

}

window.onload = main;
//*/

/*

const getIndex = (row, column) => {
  return row * width + column;
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] == Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
};

const renderLoop = () => {
  universe.tick();

  drawGrid();
  drawCells();
  requestAnimationFrame(renderLoop);
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);

*/