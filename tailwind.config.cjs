/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx}"],
  theme: {
    extend: {
      fontFamily: {
				inter: ["Inter Tight"]
			},
      colors: {
        gray: {
          900: "#121212",
        },
      },
    },
  },
  plugins: [],
};
