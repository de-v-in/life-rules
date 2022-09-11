import { cx } from "@utils/tools";
import { useCallback, useEffect } from "react";

export const AppScreen: IComponent = () => {
  const load = useCallback(async () => {
    const { RuleBuilder } = await import("@utils/builder");

    const builder = new RuleBuilder(
      document.getElementById("life") as HTMLCanvasElement
    );
    builder.set_rule();
    const update = () => {
      builder.render();
      requestAnimationFrame(update);
    };
    update();
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  return (
    <div
      className={cx(
        "bg-black w-screen h-screen flex justify-center items-center"
      )}
    >
      <canvas id="life" width="600" height="600"></canvas>
    </div>
  );
};
