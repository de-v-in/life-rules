class RuleBuilder {
  wasmModule: typeof import("@wasm") | undefined;

  constructor(canvas: HTMLCanvasElement, then?: () => void) {
    import("@wasm").then((wasm) => {
      this.wasmModule = wasm;
      this.wasmModule.main(canvas.width);
      then?.();
    });
  }

  set_rule() {
    this.wasmModule?.set_rule(
      {
        yellow: { total: 800, size: 5 },
        green: { total: 800, size: 5 },
        red: { total: 800, size: 5 },
      },
      [
        ["green", "green", "-0.32"],
        ["green", "red", "-0.17"],
        ["green", "yellow", "0.34"],
        ["red", "red", "-0.1"],
        ["red", "green", "-0.34"],
        ["yellow", "yellow", "0.15"],
        ["yellow", "green", "-0.2"],
      ]
    );
    this.wasmModule?.set_render(true);
  }

  render() {
    this.wasmModule?.start_render();
  }
}

export { RuleBuilder };
