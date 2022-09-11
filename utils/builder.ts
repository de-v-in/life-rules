import * as wasm from "@wasm";

interface IAtom {
  x: number;
  y: number;
  vx: number;
  vy: number;
  color: string;
}

type TRuleParams = [IAtom[], IAtom[], number];

class RuleBuilder {
  size = 0;
  context2D: CanvasRenderingContext2D;
  atoms: IAtom[];

  constructor(canvas: HTMLCanvasElement) {
    this.context2D = canvas.getContext("2d")!;
    this.atoms = [];
    this.size = canvas.width;
    wasm.main(canvas.width);
  }

  testWasm() {
    wasm.set_rule({ red: 100, green: 100 }, [["red", "green", "0.5"]]);
  }

  set_rule() {
    wasm.set_rule({ yellow: 400, green: 400, red: 400 }, [
      ["green", "green", "-0.32"],
      ["green", "red", "-0.17"],
      ["green", "yellow", "0.34"],
      ["red", "red", "-0.1"],
      ["red", "green", "-0.34"],
      ["yellow", "yellow", "0.15"],
      ["yellow", "green", "-0.2"],
    ]);
    wasm.set_render(true);
  }

  render() {
    wasm.start_render();
  }

  draw(x: number, y: number, color: string, size: number) {
    this.context2D.fillStyle = color;
    this.context2D.fillRect(x, y, size, size);
  }

  atom(x: number, y: number, c: string): IAtom {
    return { x: x, y: y, vx: 0, vy: 0, color: c };
  }

  random(padding = 50) {
    const size = this.size;
    return Math.random() * size - 100 + padding;
  }

  create(number: number, color: string) {
    const group = [];
    for (let i = 0; i < number; i++) {
      group.push(this.atom(this.random(), this.random(), color));
      this.atoms.push(group[i]);
    }
    return group;
  }

  rule(atoms1: IAtom[], atoms2: IAtom[], g: number) {
    // const output = JSON.parse(wasm.rule(atoms1, atoms2, g, this.size));
    // for (let i = 0; i < atoms1.length; i++) {
    //   const a = atoms1[i];
    //   atoms1[i].x = output[i].x;
    //   atoms1[i].y = output[i].y;
    //   atoms1[i].vx = output[i].vx;
    //   atoms1[i].vy = output[i].vy;
    // }
    // return;
    for (let i = 0; i < atoms1.length; i++) {
      let fx = 0;
      let fy = 0;
      let a = atoms1[0];
      let b = atoms2[0];
      for (let j = 0; j < atoms2.length; j++) {
        a = atoms1[i];
        b = atoms2[j];
        const dx = a.x - b.x;
        const dy = a.y - b.y;
        const d = Math.sqrt(dx * dx + dy * dy);
        if (d > 0 && d < 80) {
          const F = (g * 1) / d;
          fx += F * dx;
          fy += F * dy;
        }
      }
      a.vx = (a.vx + fx) * 0.5;
      a.vy = (a.vy + fy) * 0.5;
      a.x += a.vx;
      a.y += a.vy;
      if (a.x <= 0 || a.x >= this.size) {
        a.vx *= -1;
      }
      if (a.y <= 0 || a.y >= this.size) {
        a.vy *= -1;
      }
    }
  }

  async excute(params: TRuleParams[]) {
    params.forEach((param) => this.rule(...param));
    this.context2D.clearRect(0, 0, this.size, this.size);
    // this.draw(0, 0, "black", this.size);
    for (let i = 0; i < this.atoms.length; i++) {
      this.draw(this.atoms[i].x, this.atoms[i].y, this.atoms[i].color, 5);
    }
  }
}

export { RuleBuilder };
