const hoistNonReactStatics = require("hoist-non-react-statics");

/**
 * Read more here: https://github.com/vinissimus/next-translate
 */
const i18n = {
  locales: ["en", "vi"],
  defaultLocale: "en",
  pages: {
    "*": ["common"],
  },
  staticsHoc: hoistNonReactStatics,
};

module.exports = i18n;
