"use client";

import { ThemeProvider } from "@/components/theme-provider";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

let clientQueryClientSingleton: QueryClient | undefined = undefined;
const getQueryClient = () => {
	if (typeof window === "undefined") {
		return new QueryClient();
	}

	if (!clientQueryClientSingleton) {
		clientQueryClientSingleton = new QueryClient();
	}

	return clientQueryClientSingleton;
};

export function Providers({ children }: { children: React.ReactNode }) {
	const client = getQueryClient();

	return (
		<QueryClientProvider client={client}>
			<ThemeProvider>{children}</ThemeProvider>
		</QueryClientProvider>
	);
}
