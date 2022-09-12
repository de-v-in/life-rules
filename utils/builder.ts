class RuleBuilder {
  wasmModule: typeof import("@wasm") | undefined;

  constructor(canvas: HTMLCanvasElement, then?: () => void) {
    import("@wasm").then((wasm) => {
      this.wasmModule = wasm;
      this.wasmModule.main(canvas.width);
      then?.();
    });
  }

  render() {
    this.wasmModule?.start_render();
  }
}

export { RuleBuilder };
