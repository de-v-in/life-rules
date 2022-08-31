import "@styles/globals.scss";

import { MainLayout } from "@layouts/MainLayout";
import type { AppProps } from "next/app";

/**
 * Default layout for page component
 */
const DefaultLayout: IComponent = ({ children }) => <>{children}</>;

function MyApp({ Component, pageProps }: AppProps) {
  const getLayout =
    (Component as IPageComponent).getLayout ||
    ((children) => <DefaultLayout>{children}</DefaultLayout>);

  const PageContent = Component as IPageComponent;

  return <MainLayout>{getLayout(<PageContent {...pageProps} />)}</MainLayout>;
}

export default MyApp;
