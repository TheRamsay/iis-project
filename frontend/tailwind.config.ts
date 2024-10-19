import type { Config } from 'tailwindcss'

const config: Config = {
	content: [
		'./src/pages/**/*.{js,ts,jsx,tsx,mdx}',
		'./src/components/**/*.{js,ts,jsx,tsx,mdx}',
		'./src/app/**/*.{js,ts,jsx,tsx,mdx}',
	],
	theme: {
		extend: {
			animation: {
				rotate: 'rotate360 1s cubic-bezier(0.83, 0, 0.17, 1) infinite',
			},
			keyframes: {
				rotate360: {
					from: {
						transform: 'rotate(0deg)',
					},
					to: {
						transform: 'rotate(360deg)',
					},
				},
			},
			colors: {
				foreground: 'var(--foreground)',
				background: 'var(--background)',
				primary: 'var(--color)',
				secondary: 'var(--secondary)',
				muted: 'var(--muted)',
				'muted-foreground': 'var(--muted-foreground)',
				accent: 'var(--accent)',
				'accent-foreground': 'var(--accent-foreground)',
			},
		},
	},
	plugins: [],
}
export default config
