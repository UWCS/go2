/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.html"],
  darkMode: 'class',
  theme: {
    colours: {
      'uwcs-dark': '#202429'
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],

}
