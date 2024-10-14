import type { Config } from "tailwindcss";

const config: Config = {
	content: [
		"./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
		"./src/components/**/*.{js,ts,jsx,tsx,mdx}",
		"./src/app/**/*.{js,ts,jsx,tsx,mdx}",
	],
	theme: {
		extend: {
			colors: {
				foreground: "var(--foreground)",
				background: "var(--background)",
				primary: "var(--color)",
				secondary: "var(--secondary)",
				muted: "var(--muted)",
				"muted-foreground": "var(--muted-foreground)",
				accent: "var(--accent)",
				"accent-foreground": "var(--accent-foreground)",
			},
		},
	},
	plugins: [],
};
export default config;
