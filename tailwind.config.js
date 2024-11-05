/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "media",
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
        fontFamily: {
            sans: ["InterVariable", "sans-serif"],
            serif: ["Playfair", "Georgia"],
            display: ["Roca"]
        },
    extend: {},
  },
  plugins: [],
}
