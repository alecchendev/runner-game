/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Input {
  GoLeft,
  GoForward,
  GoRight,
  GoBack,
  StopLeft,
  StopForward,
  StopRight,
  StopBack,
  Jump,
  Cast,
  Pull,
  Release,
}
/**
*/
export enum Go {
  Left,
  Forward,
  Right,
  Back,
  Jump,
}
/**
*/
export class Graphics {
  free(): void;
/**
* @returns {Graphics}
*/
  static new(): Graphics;
/**
* @param {Float32Array} positions
* @param {Float32Array} colors
* @param {Uint32Array} indices
* @param {Float32Array} cam_pos
* @param {number} cam_theta
* @param {number} cam_phi
*/
  update(positions: Float32Array, colors: Float32Array, indices: Uint32Array, cam_pos: Float32Array, cam_theta: number, cam_phi: number): void;
/**
* @returns {Float32Array}
*/
  positions(): Float32Array;
/**
* @returns {Float32Array}
*/
  colors(): Float32Array;
/**
* @returns {Uint32Array}
*/
  indices(): Uint32Array;
/**
* @returns {Float32Array}
*/
  cam_pos(): Float32Array;
/**
* @returns {number}
*/
  cam_theta(): number;
/**
* @returns {number}
*/
  cam_phi(): number;
}
/**
*/
export class Universe {
  free(): void;
/**
* @returns {Universe}
*/
  static new(): Universe;
/**
* @param {number} curr_player
*/
  update(curr_player: number): void;
/**
* @param {number} curr_player
* @param {number} input
*/
  player_input(curr_player: number, input: number): void;
/**
*/
  cast_grapple(): void;
/**
*/
  pull_grapple(): void;
/**
*/
  release_grapple(): void;
/**
* @param {number} go
*/
  go(go: number): void;
/**
* @param {number} go
*/
  stop(go: number): void;
/**
* @param {number} curr_player
* @param {number} movement_x
* @param {number} movement_y
*/
  mouse_look(curr_player: number, movement_x: number, movement_y: number): void;
/**
* @returns {Graphics}
*/
  graphics(): Graphics;
}
