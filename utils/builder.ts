class RuleBuilder {
  wasmModule: typeof import("@wasm") | undefined;

  constructor(canvas: HTMLCanvasElement, then?: () => void) {
    import("@wasm").then((wasm) => {
      this.wasmModule = wasm;
      // this.wasmModule.initialize(
      //   "life",
      //   {
      //     "#4A90E2": { total: 300, size: 4, shape: "Dot", blur: 6 },
      //     "#123321": { total: 300, size: 2 },
      //   },
      //   [
      //     ["#4A90E2", "#4A90E2", "-0.32"],

      //     ["#4A90E2", "#123321", "-0.2"],

      //     ["#123321", "#4A90E2", "-0.2"],
      //   ]
      // );
      then?.();
    });
  }

  render() {
    // this.wasmModule?.start_render();
  }
}

export { RuleBuilder };
