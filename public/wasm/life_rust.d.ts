/* tslint:disable */
/* eslint-disable */
/**
* @param {string} canvas_id
* @param {any} colors
* @param {any} rules
*/
export function initialize(canvas_id: string, colors: any, rules: any): void;
/**
* @param {any} rules
*/
export function update_rules(rules: any): void;
/**
* @param {any} colors
*/
export function update_colors(colors: any): void;
/**
*/
export function start_render(): void;
/**
*/
export function stop_render(): void;
/**
*/
export function next_frame(): void;
/**
* @param {number} speed
*/
export function set_speed(speed: number): void;
/**
* @param {number} tick
*/
export function set_tick(tick: number): void;
/**
*/
export function start_loop_engine(): void;
/**
* @returns {bigint}
*/
export function get_crr_frame_idx(): bigint;
