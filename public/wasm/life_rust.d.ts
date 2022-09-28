/* tslint:disable */
/* eslint-disable */
/**
* @param {string} canvas_id
*/
export function init_engine(canvas_id: string): void;
/**
*/
export function reload(): void;
/**
*/
export function start_render(): void;
/**
*/
export function stop_render(): void;
/**
* @param {number} tick
*/
export function set_tick(tick: number): void;
/**
* @param {any} rules
*/
export function update_rules(rules: any): void;
/**
* @param {any} conf
*/
export function update_conf(conf: any): void;
/**
* @returns {bigint}
*/
export function get_crr_frame_idx(): bigint;
