'use client'

import { ThemeProvider } from '@/components/theme-provider'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { AuthProvider } from './_lib/auth/auth-provider'
import type { Session } from './_lib/auth/types'

let clientQueryClientSingleton: QueryClient | undefined = undefined
const getQueryClient = () => {
	if (typeof window === 'undefined') {
		return new QueryClient()
	}

	if (!clientQueryClientSingleton) {
		clientQueryClientSingleton = new QueryClient()
	}

	return clientQueryClientSingleton
}

export function Providers({
	children,
	session,
}: { children: React.ReactNode; session: Session | null }) {
	const client = getQueryClient()

	return (
		<QueryClientProvider client={client}>
			<ThemeProvider>
				<AuthProvider initialSession={session}>{children}</AuthProvider>
			</ThemeProvider>
		</QueryClientProvider>
	)
}
