/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: 'class',
	content: ["./**/*.{html,rs,js}"],
  theme: {
      extend: {
        colors: {
            'slate-850' : "#141f38",
        },
		gridTemplateRows: {
			'15': 'repeat(15, minmax(0, 1fr))',
		},
        gridTemplateColumns: {
			'15': 'repeat(15, minmax(0, 1fr))',
		},
      },
  },
  plugins: [],
}

