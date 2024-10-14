"use client";

import { ThemeProvider as NextThemeProvider } from "next-themes";
import type { FC, ReactNode } from "react";

interface ThemeProviderProps {
	children: ReactNode | ReactNode[];
	forcedTheme?: string;
}

export const ThemeProvider: FC<ThemeProviderProps> = ({
	children,
	forcedTheme,
}) => {
	return (
		<NextThemeProvider
			attribute="class"
			disableTransitionOnChange
			forcedTheme={forcedTheme}
		>
			<div id="network-check-portal" />
			{children}
			<div id="popover-portal" />
			<div id="footer-portal" />
		</NextThemeProvider>
	);
};
