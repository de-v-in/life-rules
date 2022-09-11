import Tabs, {
  Tab,
  TabPanel,
  TabsBody,
  TabsHeader,
} from "@material-tailwind/react/components/Tabs";
import { RuleBuilder } from "@utils/builder";
import { cx } from "@utils/tools";
import { useCallback, useEffect, useRef, useState } from "react";

const FPS_RPOFILE = [1, 5, 10, 30, 60, -1, 0];

export const AppScreen: IComponent = () => {
  const [speed, setSpeed] = useState<number>(30);
  const [loaded, setLoaded] = useState(false);
  const builderRef = useRef<RuleBuilder | null>(null);
  const maxRef = useRef(false);

  const load = useCallback(async () => {
    const builder = new RuleBuilder(
      document.getElementById("life") as HTMLCanvasElement,
      () => {
        builder.set_rule();
        builderRef.current = builder;
        setLoaded(true);
      }
    );
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  useEffect(() => {
    const update = () => {
      builderRef.current?.render();
      if (maxRef.current) {
        requestAnimationFrame(update);
      }
    };
    let renewer: NodeJS.Timer;

    if (speed === 0) {
      maxRef.current = false;
      return;
    }

    if (speed === -1) {
      if (loaded) {
        maxRef.current = true;
        update();
      }
    } else {
      maxRef.current = false;
      renewer = setInterval(() => {
        if (loaded) {
          requestAnimationFrame(update);
        }
      }, 1000 / speed);
    }

    return () => {
      renewer && clearInterval(renewer);
    };
  }, [loaded, speed]);

  return (
    <div
      className={cx(
        "bg-black w-screen h-screen flex justify-center items-center"
      )}
    >
      <div className="absolute top-1 right-1 flex flex-row text-white items-center gap-2">
        <Tabs id="custom-animation" value={speed}>
          <TabsHeader className="rounded-none bg-transparent">
            <div className="h-auto flex justify-center items-center px-2 bg-white text-gray-800 rounded mr-2">
              <span className="text-xs font-bold">FPS</span>
            </div>
            {FPS_RPOFILE.map((label) => (
              <Tab
                key={label}
                value={label}
                className={cx("px-2 text-xs text-white", {
                  "text-green-500 font-bold": label === speed,
                })}
                onClick={() => setSpeed(label)}
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
          </TabsHeader>
        </Tabs>
      </div>
      <canvas id="life" width="600" height="600"></canvas>
    </div>
  );
};
