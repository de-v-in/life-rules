import { useRouter } from "next/router";

import { TransitionLayout } from "./TransitionLayout";

export const DefaultLayout: IComponent = ({ children }) => {
  const router = useRouter();
  return (
    <div className="w-full h-full overflow-x-hidden">
      <TransitionLayout location={router.pathname}>
        <>{children}</>
      </TransitionLayout>
    </div>
  );
};
