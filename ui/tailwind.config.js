/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    "./node_modules/primevue/**/*.{vue,js,ts,jsx,tsx}"
  ],
  theme: {
    fontFamily: {
      sans: ["montserrat", ...defaultTheme.fontFamily.sans]
    },
    extend: {
      rotate: {
        '15': '15deg'
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

