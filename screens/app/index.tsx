import { FPSController } from "@components/FPSController";
import IconButton from "@material-tailwind/react/components/IconButton";
import TeamLogo from "@public/team.png";
import { useBuilderStore } from "@states/builder";
import { RuleBuilder } from "@utils/builder";
import { cx } from "@utils/tools";
import Image from "next/image";
import { useCallback, useEffect, useRef, useState } from "react";

import { ColorConfiguration } from "./ColorConfiguration";
import { RuleConfiguration } from "./RuleConfiguration";

export const AppScreen: IComponent = () => {
  const [loaded, setLoaded] = useState(false);
  const builderRef = useRef<RuleBuilder | null>(null);
  const { colors, rules, getWASMRule, changePickerStatus } = useBuilderStore();

  const initialBuilder = useCallback(async () => {
    const builder = new RuleBuilder(
      document.getElementById("life") as HTMLCanvasElement,
      () => {
        builder.wasmModule?.init_engine("life");
        builder.wasmModule?.update_conf(colors);
        builder.wasmModule?.update_rules(getWASMRule());
        builder.wasmModule?.change_entropy(0.8);
        builder.wasmModule?.start_render();
        builderRef.current = builder;
        setLoaded(true);
      }
    );
  }, []);

  const triggerRender = useCallback(() => {
    // builderRef.current?.wasmModule?.next_frame();
  }, []);
  const updateTick = (tick: number) => {
    if (tick === 0) {
      builderRef.current?.wasmModule?.stop_render();
    } else {
      builderRef.current?.wasmModule?.start_render();
      builderRef.current?.wasmModule?.set_tick(tick);
    }
  };
  const showColorPicker = () => changePickerStatus(true);

  const reloadCanvas = useCallback(() => {
    builderRef.current?.wasmModule?.reload();
  }, []);

  const reloadRule = useCallback(() => {
    builderRef.current?.wasmModule?.update_rules(getWASMRule());
  }, [getWASMRule]);

  useEffect(() => {
    initialBuilder();
  }, [initialBuilder]);

  useEffect(() => {
    // TODO: Update while rendering when it ready
    // builderRef.current?.wasmModule?.update_conf(colors);
    // FIX: Current we reinitial canvas when update color configs
    // reloadCanvas();
    builderRef.current?.wasmModule?.update_conf(colors);
  }, [colors]);

  useEffect(() => {
    reloadRule();
  }, [reloadRule, rules]);

  return (
    <div className="flex flex-row h-full bg-black">
      <div style={{ minWidth: 400 }} className="h-full flex-col gap-2 p-2 flex">
        <FPSController
          enable={loaded}
          getFrameIdx={builderRef.current?.wasmModule?.get_crr_frame_idx}
          updateTick={updateTick}
          reloadFn={reloadCanvas}
        />
        <div className="w-full h-1/3 border border-blue-900 flex flex-col">
          <div className="bg-blue-900 p-1 flex justify-center items-center">
            <p className="text-xs text-white text-source flex-auto">
              COLOR CONFIGURATIONS
            </p>
            <IconButton
              color="white"
              variant="text"
              size="sm"
              onClick={showColorPicker}
              style={{ height: 18, width: 18 }}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth="1.5"
                stroke="white"
                className="w-4 h-4"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M12 4.5v15m7.5-7.5h-15"
                />
              </svg>
            </IconButton>
          </div>
          <ColorConfiguration />
        </div>
        <div className="w-full flex-auto border border-blue-900 flex flex-col">
          <div className="bg-blue-900 p-1">
            <p className="text-xs text-white text-source">
              RULE CONFIGURATIONS
            </p>
          </div>
          <RuleConfiguration />
        </div>
      </div>
      <div
        className={cx(
          "w-screen h-screen flex justify-center items-center relative"
        )}
      >
        <div className="absolute top-2 left-2"></div>
        <canvas
          id="life"
          width="600px"
          height="600px"
          className="border border-gray-800 rounded-lg"
        ></canvas>
        {/* <div className="absolute bottom-3 right-3"> */}
        {/*   <div className="relative"> */}
        {/*     <Image */}
        {/*       src={TeamLogo} */}
        {/*       alt="team" */}
        {/*       layout="fixed" */}
        {/*       width={96} */}
        {/*       height={32} */}
        {/*       objectFit="contain" */}
        {/*     /> */}
        {/*   </div> */}
        {/* </div> */}
      </div>
    </div>
  );
};
