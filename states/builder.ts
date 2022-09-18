import create from "zustand";

interface IBuilderState {
  showPicker: boolean;
  colors: {
    [key: string]: {
      total: number;
      size: number;
      shape?: string;
      blur?: number;
    };
  };
  rules: [string, string, number][];
  getWASMRule: () => string[][];
  addColor: (name: string) => void;
  editColorTotal: (name: string, total: number) => void;
  editColorSize: (name: string, size: number) => void;
  removeColor: (name: string) => void;
  changePickerStatus: (show: boolean) => void;
  updateRule: (conf: [string, string, number]) => void;
  removeRule: (identity: [string, string]) => void;
}

const useBuilderStore = create<IBuilderState>((set, get) => ({
  showPicker: false,
  colors: {
    "#4A90E2": { total: 300, size: 6, shape: "Square" },
    "#ffffff": { total: 300, size: 1, shape: "Square" },
  },
  rules: [
    ["#4A90E2", "#4A90E2", -0.32],
    ["#4A90E2", "#ffffff", 0.34],
    ["#ffffff", "#ffffff", 0.15],
    ["#ffffff", "#4A90E2", -0.2],
  ],
  getWASMRule: () => {
    return get()
      .rules.filter((v) => v[2] !== 0.0)
      .map((v) => v.map((parms) => parms.toString()));
  },
  addColor: (name) => {
    const oldColors = Object.keys(get().colors);
    set({
      colors: {
        ...get().colors,
        [name]: { total: 300, size: 4 },
      },
      rules: [
        ...get().rules,
        ...oldColors.map((v) => [v, name, 0.0] as [string, string, number]),
        ...oldColors.map((v) => [name, v, 0.0] as [string, string, number]),
        [name, name, 0.0],
      ].sort(
        (a, b) =>
          oldColors.indexOf(b[0] as string) - oldColors.indexOf(a[0] as string)
      ) as [string, string, number][],
    });
  },
  removeColor: (color: string) => {
    const colors = get().colors;
    delete colors[color];
    const rules = get().rules.filter((v) => v[0] !== color && v[1] !== color);
    set({
      colors: { ...colors },
      rules,
    });
  },
  editColorTotal: (name, total) => {
    set({
      colors: {
        ...get().colors,
        [name]: { ...get().colors[name], total },
      },
    });
  },
  editColorSize: (name, size) => {
    set({
      colors: {
        ...get().colors,
        [name]: { ...get().colors[name], size },
      },
    });
  },
  changePickerStatus: (show) => {
    set({ showPicker: show });
  },
  updateRule: (newRule) => {
    set({
      rules: get().rules.map((v) => {
        if (v[0] === newRule[0] && v[1] === newRule[1]) {
          v[2] = newRule[2];
        }
        return v;
      }),
    });
  },
  removeRule: (id) => {
    set({
      rules: get().rules.map((v) => {
        if (v[0] === id[0] && v[1] === id[1]) {
          v[2] = 0;
        }
        return v;
      }),
    });
  },
}));

export { useBuilderStore };
