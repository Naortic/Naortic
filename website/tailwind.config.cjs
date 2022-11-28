/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {}
	},
	plugins: [require('daisyui')],

	daisyui: {
		themes: [
			{
				luxury: {
					...require('daisyui/src/colors/themes')[
						'[data-theme=luxury]'
					],
					'base-300': '#0f0f11'
				}
			}
		]
	}
};
