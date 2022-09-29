const withMT = require("@material-tailwind/react/utils/withMT");

module.exports = withMT({
  darkMode: "class",
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./layouts/**/*.{js,ts,jsx,tsx}",
    "./screens/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
    "./node_modules/react-simple-tailwind-table/**/*.{html,js,ts,css,scss}",
  ],
  theme: {
    extend: {
      fontSize: { xxs: ".8rem" },
      boxShadow: {
        "inner-md": "inset 0px 0px 4px #00000020",
        "inner-xl": "inset 0px 0px 8px #00000020",
        "inner-2xl": "inset 0px 0px 16px #00000040",
      },
    },
  },
  plugins: [],
});
