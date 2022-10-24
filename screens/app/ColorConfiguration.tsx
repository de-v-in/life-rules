import "react-simple-tailwind-table/build/style.css";

import Button from "@material-tailwind/react/components/Button";
import Dialog, {
  DialogBody,
  DialogHeader,
} from "@material-tailwind/react/components/Dialog";
import IconButton from "@material-tailwind/react/components/IconButton";
import { useBuilderStore } from "@states/builder";
import { useState } from "react";
import { SketchPicker } from "react-color";
import {
  TailwindTable,
  useTableConfiguration,
} from "react-simple-tailwind-table";

export const ColorConfiguration: IComponent = () => {
  const [selectedColor, setSelectedColor] = useState("#ffffff");

  const {
    showPicker,
    changePickerStatus,
    colors,
    editColorTotal,
    editColorSize,
    removeColor,
    addColor,
  } = useBuilderStore();

  const handleHidePicker = () => changePickerStatus(false);
  const handleShowPicker = () => changePickerStatus(true);

  const handleAddNewColor = () => {
    if (!colors[selectedColor]) {
      addColor(selectedColor);
      changePickerStatus(false);
    }
  };

  const { tableColumns, tableData } = useTableConfiguration(
    Object.keys(colors).map((k) => ({ name: k, ...colors[k] })),
    [
      {
        label: "NAME",
        accessor: "name",
        header: {
          background: "#0a3880",
          className:
            "border-blue-800 text-white text-source border-l-transparent",
        },
        body: {
          background: "#222",
          className:
            "border-blue-800 text-white text-xs py-1 border-l-transparent",
        },
        align: "left",
        renderData: (data) => {
          return (
            <div className="flex items-center px-2">
              <div
                style={{ background: data.name, width: 2, height: 12 }}
                className="mr-2 rounded"
              />
              <span>{data.name.toLocaleUpperCase()}</span>
            </div>
          );
        },
      },
      {
        label: "Total Points",
        accessor: "total",
        header: {
          background: "#0a3880",
          className: "border-blue-800 text-white text-source",
        },
        body: {
          background: "#222",
          className: "border-blue-800 text-white text-xs",
        },
        width: 100,
        renderData: (data) => {
          return (
            <input
              style={{ width: 100 }}
              className="bg-transparent text-white px-2 text-xs py-1 outline-none text-center"
              defaultValue={data.total}
              onChange={(ev) =>
                editColorTotal(data.name, parseInt(ev.target.value, 10) || 0)
              }
              name="total"
            />
          );
        },
      },
      {
        label: "Point size",
        accessor: "point_size",
        header: {
          background: "#0a3880",
          className: "border-blue-800 text-white text-source",
        },
        body: {
          background: "#222",
          className: "border-blue-800 text-white text-xs",
        },
        renderHeader: () => {
          return (
            <span className="text-xs">
              POINT SIZE <strong className="opacity-50 font-bold">PX</strong>
            </span>
          );
        },
        width: 100,
        renderData: (data) => {
          return (
            <input
              style={{ width: 100 }}
              className="w-32 bg-transparent text-white px-2 text-xs py-1 outline-none text-center"
              defaultValue={data.point_size}
              onChange={(ev) =>
                editColorSize(data.name, parseInt(ev.target.value, 10) || 0)
              }
              name="total"
            />
          );
        },
      },
      {
        label: "",
        header: {
          background: "#0a3880",
          className:
            "border-blue-800 text-white text-source border-r-transparent",
        },
        body: {
          background: "#222",
          className: "border-blue-800 text-white text-xs border-r-transparent",
        },
        width: 32,
        renderData: (data) => {
          return (
            <IconButton
              size="sm"
              color="white"
              variant="text"
              className="w-full rounded-none px-4"
              onClick={() => removeColor(data.name)}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth={1.5}
                stroke="red"
                className="w-4 h-4"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </IconButton>
          );
        },
      },
    ]
  );

  return (
    <div className="flex h-full relative">
      <div className="absolute top-0 left-0 w-full flex flex-col gap-1 max-h-full overflow-auto">
        {/* <div className="flex flex-row w-full gap-1">
        <div className="w-1/3 text-white px-2 text-xs py-1 text-source font-bold opacity-90">
          COLOR
        </div>
        <div className="w-1/3 text-white px-2 text-xs py-1 text-source font-bold opacity-90">
          TOTAL
        </div>
        <div className="w-1/3 text-white px-2 text-xs py-1 text-source font-bold opacity-90">
          SIZE
        </div>
      </div>
      {renderColors} */}
        <TailwindTable
          data={tableData}
          columns={tableColumns}
          className="border-blue-800"
          difference={{ enable: true, offset: 1 }}
        />
        <Dialog
          className="rounded-none bg-black border-blue-800 border outline-none"
          open={showPicker}
          handler={handleShowPicker}
        >
          <DialogHeader className="bg-blue-800 text-white text-xs py-1">
            <div className="flex w-full justify-center items-center gap-2">
              <span className="flex-auto">SELECT YOUR NEW COLOR</span>
              <Button
                onClick={handleHidePicker}
                size="sm"
                color="white"
                variant="text"
                style={{ height: 26, padding: 0 }}
              >
                <span className="px-1">BACK</span>
              </Button>
              <Button
                onClick={handleAddNewColor}
                size="sm"
                color="white"
                variant="text"
                style={{ height: 26, padding: 0 }}
              >
                <span className="text-white px-1">ADD</span>
              </Button>
            </div>
          </DialogHeader>
          <DialogBody
            divider
            className="border-none flex justify-center items-center p-0"
          >
            <SketchPicker
              width="100%"
              color={selectedColor}
              onChange={(color) => setSelectedColor(color.hex)}
              styles={{
                default: {
                  picker: {
                    background: "black",
                    color: "black",
                    border: "none",
                    borderRadius: 0,
                  },
                },
              }}
            />
          </DialogBody>
        </Dialog>
      </div>
    </div>
  );
};
