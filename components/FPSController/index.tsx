import Tabs, {
  Tab,
  TabsHeader,
} from "@material-tailwind/react/components/Tabs";
import { Browser } from "@saintno/needed-tools";
import { cx } from "@utils/tools";
import { useCallback, useEffect, useRef, useState } from "react";

import styles from "./styles.module.scss";

const FPS_RPOFILE = [1, 5, 10, 15, 30, -1, 0];

export const FPSController: IComponent<{
  enable?: boolean;
  frameRenderFn?: () => void;
  reloadFn?: () => void;
}> = ({ enable = false, frameRenderFn, reloadFn }) => {
  const [speed, setSpeed] = useState<number>(30);
  const [fps, setFps] = useState(30);
  const crrSpeed = useRef(30);
  const maxRef = useRef(false);
  const ticks = useRef<number[]>([]);

  const handleChangeSpeed = useCallback((nSpeed: number) => {
    setSpeed(nSpeed);
    crrSpeed.current = nSpeed;
    if (nSpeed >= 0) {
      maxRef.current = false;
      setFps(nSpeed);
    }
    if (nSpeed === -1) maxRef.current = true;
  }, []);

  const startRender = useCallback(() => {
    if (!enable) return;
    if (crrSpeed.current !== speed) return;
    if (crrSpeed.current === 0) return;
    const now = performance.now();
    while (ticks.current.length > 0 && ticks.current[0] <= now - 1000) {
      ticks.current.shift();
    }
    setFps(ticks.current.length);
    frameRenderFn?.();
    if (maxRef.current) {
      ticks.current.push(now);
      requestAnimationFrame(startRender);
    } else {
      const nextTick =
        1000 / speed +
        ticks.current[ticks.current.length - 1] -
        performance.now();
      ticks.current.push(performance.now());
      if (nextTick <= 0) {
        requestAnimationFrame(startRender);
      } else {
        setTimeout(() => {
          requestAnimationFrame(startRender);
        }, nextTick);
      }
    }
  }, [enable, frameRenderFn, speed]);

  const handleTabChange = useCallback(() => {
    if (document.visibilityState == "visible") {
      crrSpeed.current = speed;
      if (Browser.isSafari) {
        startRender();
      }
    } else {
      crrSpeed.current = 0;
    }
  }, [speed, startRender]);

  useEffect(() => {
    startRender();
  }, [startRender]);

  useEffect(() => {
    document.addEventListener("visibilitychange", handleTabChange);
    return () => {
      document.removeEventListener("visibilitychange", handleTabChange);
    };
  }, [handleTabChange]);

  return (
    <>
      <div className="absolute top-2 right-2 flex flex-col items-end">
        <span className="text-white text-craft text-xl opacity-80">
          RULE<span className="opacity-70 text-sm">OF</span>
          <span className="text-blue-500">PARTICLES</span>
        </span>
        <span className="text-white text-craft opacity-90">
          {fps} <span className="opacity-50">FPS</span>
        </span>
      </div>
      <Tabs id="custom-animation" value={speed}>
        <TabsHeader
          indicatorProps={{ className: "rounded-none bg-gray-800" }}
          className="rounded-none bg-transparent border border-blue-900 p-0"
        >
          <div className="h-auto flex justify-center items-center px-2 bg-blue-900 text-white rounded-none">
            <span className="text-xs font-bold whitespace-nowrap">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                className="w-3 h-3"
              >
                <path
                  fillRule="evenodd"
                  d="M14.615 1.595a.75.75 0 01.359.852L12.982 9.75h7.268a.75.75 0 01.548 1.262l-10.5 11.25a.75.75 0 01-1.272-.71l1.992-7.302H3.75a.75.75 0 01-.548-1.262l10.5-11.25a.75.75 0 01.913-.143z"
                  clipRule="evenodd"
                />
              </svg>
            </span>
          </div>
          {FPS_RPOFILE.map((label) => (
            <Tab
              key={label}
              value={label}
              className={cx("px-2 text-xs text-white", {
                "text-blue-500 font-bold": label === speed,
              })}
              onClick={() => handleChangeSpeed(label)}
            >
              {label === 0 ? (
                <span className="text-orange-500">PAUSE</span>
              ) : label === -1 ? (
                "MAX"
              ) : (
                label
              )}
            </Tab>
          ))}
          <button
            onClick={reloadFn}
            className="h-auto flex justify-center items-center px-2 bg-blue-900 text-white rounded-none active:opacity-90"
          >
            <span className="text-xs font-bold whitespace-nowrap">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth={1.5}
                stroke="currentColor"
                className="w-3 h-3"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
                />
              </svg>
            </span>
          </button>
        </TabsHeader>
      </Tabs>
    </>
  );
};
