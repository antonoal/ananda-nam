/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    "./node_modules/primevue/**/*.{vue,js,ts,jsx,tsx}"
  ],
  theme: {
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

