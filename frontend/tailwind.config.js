/** @type {import('tailwindcss').Config} */

module.exports = {
  content: ["./src/**/*.{html,js}"],
  theme: {
    fontFamily: {
      sans: ["Roboto", "sans-serif"],
      gsans: ["Google Sans", "sans-serif"],
    },
    extend: {
      colors: {
        mainblack: "#181a1b",
        headergray: "#1c1f21",
        darkgrey: "#3c4043",
        boxBlue: "#2c52ba",
      },
    },
  },
  plugins: [],
};
