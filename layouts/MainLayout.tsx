import { useAppStore } from "@states/app";
import { cx } from "@utils/tools";

export const MainLayout: IComponent = ({ children }) => {
  // Manual switch darkmode with state
  const { darkMode } = useAppStore();

  return (
    <div
      className={cx(
        {
          dark: darkMode === "dark",
        },
        "w-full h-full"
      )}
    >
      {/* Smooth out darkmode transition with transition-all */}
      <div className="w-full h-full bg-white dark:bg-black transition-all">
        {children}
      </div>
    </div>
  );
};
