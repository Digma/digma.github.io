/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      fontFamily: {
        resume: ["Outfit", "Open sans", "sans-serif"],
        blog: ["Open sans", "sans-serif"],
      },
      typography: {
        DEFAULT: {
          css: {
            maxWidth: "860px",
          },
        },
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
