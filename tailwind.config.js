/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: 'class',
	content: ["./**/*.{html,rs}"],
  theme: {
      extend: {
        colors: {
            'slate-850' : "#141f38",
        },
			gridTemplateRows {
				'10': 'repeat(10, minmax(0, 1fr))',
			}
      },
  },
  plugins: [],
}

