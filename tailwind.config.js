/** @type {import('tailwindcss').Config} */

const { nextui } = require('@nextui-org/react');

export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}', './node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}'],
  theme: {
    fontFamily: {
      title: ['Bluu Next Bold'],
    },
    extend: {},
  },
  darkMode: 'class',
  plugins: [
    nextui({
      themes: {
        light: {
          colors: {
            primary: {
              DEFAULT: '#b45309',
            },
          },
        },
        dark: {
          colors: {
            primary: {
              DEFAULT: '#b45309',
            },
          },
        },
      },
    }),
  ],
};
