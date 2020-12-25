import * as wasm from './runner_game_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getFloat32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    return getUint32Memory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
*/
export const Input = Object.freeze({ GoLeft:0,"0":"GoLeft",GoForward:1,"1":"GoForward",GoRight:2,"2":"GoRight",GoBack:3,"3":"GoBack",StopLeft:4,"4":"StopLeft",StopForward:5,"5":"StopForward",StopRight:6,"6":"StopRight",StopBack:7,"7":"StopBack",Jump:8,"8":"Jump",Cast:9,"9":"Cast",Pull:10,"10":"Pull",Release:11,"11":"Release", });
/**
*/
export const Go = Object.freeze({ Left:0,"0":"Left",Forward:1,"1":"Forward",Right:2,"2":"Right",Back:3,"3":"Back",Jump:4,"4":"Jump", });
/**
*/
export class Graphics {

    static __wrap(ptr) {
        const obj = Object.create(Graphics.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_graphics_free(ptr);
    }
    /**
    * @returns {Graphics}
    */
    static new() {
        var ret = wasm.graphics_new();
        return Graphics.__wrap(ret);
    }
    /**
    * @param {Float32Array} positions
    * @param {Float32Array} colors
    * @param {Uint32Array} indices
    * @param {Float32Array} cam_pos
    * @param {number} cam_theta
    * @param {number} cam_phi
    */
    update(positions, colors, indices, cam_pos, cam_theta, cam_phi) {
        var ptr0 = passArrayF32ToWasm0(positions, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArrayF32ToWasm0(colors, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = passArray32ToWasm0(indices, wasm.__wbindgen_malloc);
        var len2 = WASM_VECTOR_LEN;
        var ptr3 = passArrayF32ToWasm0(cam_pos, wasm.__wbindgen_malloc);
        var len3 = WASM_VECTOR_LEN;
        wasm.graphics_update(this.ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, cam_theta, cam_phi);
    }
    /**
    * @returns {Float32Array}
    */
    positions() {
        try {
            const retptr = wasm.__wbindgen_export_1.value - 16;
            wasm.__wbindgen_export_1.value = retptr;
            wasm.graphics_positions(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v0;
        } finally {
            wasm.__wbindgen_export_1.value += 16;
        }
    }
    /**
    * @returns {Float32Array}
    */
    colors() {
        try {
            const retptr = wasm.__wbindgen_export_1.value - 16;
            wasm.__wbindgen_export_1.value = retptr;
            wasm.graphics_colors(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v0;
        } finally {
            wasm.__wbindgen_export_1.value += 16;
        }
    }
    /**
    * @returns {Uint32Array}
    */
    indices() {
        try {
            const retptr = wasm.__wbindgen_export_1.value - 16;
            wasm.__wbindgen_export_1.value = retptr;
            wasm.graphics_indices(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v0;
        } finally {
            wasm.__wbindgen_export_1.value += 16;
        }
    }
    /**
    * @returns {Float32Array}
    */
    cam_pos() {
        try {
            const retptr = wasm.__wbindgen_export_1.value - 16;
            wasm.__wbindgen_export_1.value = retptr;
            wasm.graphics_cam_pos(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v0;
        } finally {
            wasm.__wbindgen_export_1.value += 16;
        }
    }
    /**
    * @returns {number}
    */
    cam_theta() {
        var ret = wasm.graphics_cam_theta(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    cam_phi() {
        var ret = wasm.graphics_cam_phi(this.ptr);
        return ret;
    }
}
/**
*/
export class Universe {

    static __wrap(ptr) {
        const obj = Object.create(Universe.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_universe_free(ptr);
    }
    /**
    * @returns {Universe}
    */
    static new() {
        var ret = wasm.universe_new();
        return Universe.__wrap(ret);
    }
    /**
    * @param {number} curr_player
    */
    update(curr_player) {
        wasm.universe_update(this.ptr, curr_player);
    }
    /**
    * @param {number} curr_player
    * @param {number} input
    */
    player_input(curr_player, input) {
        wasm.universe_player_input(this.ptr, curr_player, input);
    }
    /**
    */
    cast_grapple() {
        wasm.universe_cast_grapple(this.ptr);
    }
    /**
    */
    pull_grapple() {
        wasm.universe_pull_grapple(this.ptr);
    }
    /**
    */
    release_grapple() {
        wasm.universe_release_grapple(this.ptr);
    }
    /**
    * @param {number} go
    */
    go(go) {
        wasm.universe_go(this.ptr, go);
    }
    /**
    * @param {number} go
    */
    stop(go) {
        wasm.universe_stop(this.ptr, go);
    }
    /**
    * @param {number} curr_player
    * @param {number} movement_x
    * @param {number} movement_y
    */
    mouse_look(curr_player, movement_x, movement_y) {
        wasm.universe_mouse_look(this.ptr, curr_player, movement_x, movement_y);
    }
    /**
    * @returns {Graphics}
    */
    graphics() {
        var ret = wasm.universe_graphics(this.ptr);
        return Graphics.__wrap(ret);
    }
}

export const __wbg_log_447618b456770fdc = function(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

