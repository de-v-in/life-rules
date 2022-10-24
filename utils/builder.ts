class RuleBuilder {
  wasmModule: typeof import("@wasm") | undefined;

  constructor(canvas: HTMLCanvasElement, then?: () => void) {
    import("@wasm").then((wasm) => {
      this.wasmModule = wasm;
      then?.();
    });
  }
}

export { RuleBuilder };
