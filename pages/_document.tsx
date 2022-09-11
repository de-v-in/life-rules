import { Head, Html, Main, NextScript } from "next/document";

const Document = () => {
  return (
    <Html>
      <Head>
        <link
          rel="stylesheet"
          href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"
        />
        <link
          rel="stylesheet"
          href="https://fonts.googleapis.com/icon?family=Material+Icons"
        />
      </Head>
      <body>
        <script
          type="module"
          dangerouslySetInnerHTML={{
            __html: `import {rule} from "./wasm/release.js"; window.wasm = rule;`,
          }}
        ></script>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
};

export default Document;
