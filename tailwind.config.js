/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./static/**/*.html", "templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        uwcs: "#202429",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
