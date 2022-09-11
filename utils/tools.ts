import classNames from "classnames";

/**
 * Mapping hotkey into className package for better usage
 */
const cx = classNames;

/**
 * Delay current function by given time
 * @param {number} ms Time delayed
 */
const letDelay = (ms: number): Promise<boolean> => {
  return new Promise((rs) => {
    setTimeout(() => {
      rs(true);
    }, ms);
  });
};

export { cx, letDelay };
