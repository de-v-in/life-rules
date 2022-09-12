import IconButton from "@material-tailwind/react/components/IconButton";
import Tooltip from "@material-tailwind/react/components/Tooltip";
import { useBuilderStore } from "@states/builder";
import { cx } from "@utils/tools";
import { ChangeEventHandler, useMemo, useState } from "react";
import {
  TailwindTable,
  useTableConfiguration,
} from "react-simple-tailwind-table";

export const RuleConfiguration: IComponent = () => {
  const { rules, updateRule, removeRule } = useBuilderStore();
  const [filterText, setFilterText] = useState("");

  const handleChangeSearchText: ChangeEventHandler<HTMLInputElement> = (ev) => {
    setFilterText(ev.target.value);
  };

  const { tableData, tableColumns } = useTableConfiguration(
    rules.map((v) => ({
      colorA: v[0],
      colorB: v[1],
      weight: v[2],
    })),
    [
      {
        label: "Source color",
        accessor: "colorA",
        header: {
          background: "#0a3880",
          className: "border-blue-800 text-white text-source",
        },
        body: {
          background: "#222",
          className: "border-blue-800 text-white text-xs py-1",
        },
        align: "left",
        renderData: (data) => {
          return (
            <div className="flex items-center px-2">
              <div
                style={{ background: data.colorA, width: 2, height: 12 }}
                className="mr-2 rounded"
              />
              <span>{data.colorA.toLocaleUpperCase()}</span>
            </div>
          );
        },
      },
      {
        label: "Target color",
        accessor: "colorB",
        header: {
          background: "#0a3880",
          className: "border-blue-800 text-white text-source",
        },
        body: {
          background: "#222",
          className: "border-blue-800 text-white text-xs py-1",
        },
        align: "left",
        renderData: (data) => {
          return (
            <div className="flex items-center px-2">
              <div
                style={{ background: data.colorB, width: 2, height: 12 }}
                className="mr-2 rounded"
              />
              <span>{data.colorB.toLocaleUpperCase()}</span>
            </div>
          );
        },
      },
      {
        label: "GRAVITY",
        accessor: "weight",
        header: {
          background: "#0a3880",
          className: "border-blue-800 text-white text-source",
        },
        body: {
          background: "#222",
          className: "border-blue-800 text-white text-xs py-1",
        },
        align: "right",
        width: 100,
        renderHeader: () => {
          return (
            <Tooltip
              className="border-blue-500 border rounded-none"
              content={
                <div className="flex flex-col text-xxs">
                  <p className="font-bold">
                    FROM RANGE{" "}
                    <span className="text-green-500">0.00 TO 1.00</span> AND{" "}
                    <span className="text-orange-500">0.00 TO -1.00</span>
                  </p>
                  <span className="text-orange-500">
                    Lower than 0 will cause source&apos;s particle move to
                    target
                  </span>
                  <span className="text-green-500">
                    Higher than 0 will cause source&apos;s particle move away
                    from target
                  </span>
                  <span>Far from 0 mean move faster</span>
                </div>
              }
            >
              <div className="px-2 flex gap-2 justify-end items-center">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  className="w-3 h-3"
                >
                  <path
                    fillRule="evenodd"
                    d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm8.706-1.442c1.146-.573 2.437.463 2.126 1.706l-.709 2.836.042-.02a.75.75 0 01.67 1.34l-.04.022c-1.147.573-2.438-.463-2.127-1.706l.71-2.836-.042.02a.75.75 0 11-.671-1.34l.041-.022zM12 9a.75.75 0 100-1.5.75.75 0 000 1.5z"
                    clipRule="evenodd"
                  />
                </svg>
                <span className="text-xs flex-auto">GRAVITY</span>
              </div>
            </Tooltip>
          );
        },
        renderData: (data) => {
          const handleUpdateThisRule: ChangeEventHandler<HTMLInputElement> = (
            ev
          ) => {
            updateRule([
              data.colorA,
              data.colorB,
              parseFloat(ev.target.value) || 0,
            ]);
          };
          return (
            <div className="flex items-center justify-end">
              <input
                style={{ width: 100 }}
                defaultValue={data.weight}
                onChange={handleUpdateThisRule}
                className={cx("bg-transparent px-2 text-right outline-none", {
                  "text-orange-500": data.weight < 0,
                  "text-green-500": data.weight > 0,
                  "text-gray-400": data.weight === 0,
                })}
              />
              {data.weight < 0 && (
                <div
                  style={{
                    width: 2,
                    minWidth: 2,
                    height: 12,
                    opacity: Math.abs(data.weight),
                  }}
                  className="mr-2 rounded bg-orange-500"
                />
              )}
              {data.weight > 0 && (
                <div
                  style={{
                    width: 2,
                    minWidth: 2,
                    height: 12,
                    opacity: Math.abs(data.weight),
                  }}
                  className="mr-2 rounded bg-green-500"
                />
              )}
              {data.weight === 0 && (
                <div
                  style={{
                    width: 2,
                    minWidth: 2,
                    height: 12,
                    opacity: 0.1,
                  }}
                  className="mr-2 rounded bg-white"
                />
              )}
            </div>
          );
        },
      },
      {
        label: "",
        accessor: "tools",
        header: {
          background: "#0a3880",
          className: "border-blue-800 text-white text-source",
        },
        body: {
          background: "#222",
          className: "border-blue-800 text-white text-xs",
        },
        width: 32,
        renderData: (data) => {
          return (
            <IconButton
              size="sm"
              variant="text"
              color="white"
              className="w-full rounded-none px-4"
              onClick={() => removeRule([data.colorA, data.colorB])}
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
  const filteredData = useMemo(() => {
    if (filterText.length === 0) return tableData;
    return tableData.filter(
      (v) =>
        v.colorA.toLowerCase().includes(filterText.toLowerCase()) ||
        v.colorB.toLowerCase().includes(filterText.toLowerCase())
    );
  }, [tableData, filterText]);

  return (
    <>
      <div className="flex h-full relative">
        <div className="absolute top-0 left-0 w-full flex flex-col gap-1 max-h-full overflow-auto">
          <TailwindTable
            data={filteredData}
            columns={tableColumns}
            className="border-blue-800 overflow-auto h-auto"
            difference={{ enable: true, offset: 1 }}
          />
        </div>
      </div>
      <div className="mb-1 w-full border-t border-blue-800 py-1">
        <input
          onChange={handleChangeSearchText}
          placeholder="ENTER COLOR NAME TO FILTER..."
          className="w-full h-full px-2 bg-black text-white text-source text-xs outline-none"
        />
      </div>
    </>
  );
};
