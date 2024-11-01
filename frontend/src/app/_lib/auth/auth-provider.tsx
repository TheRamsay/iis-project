'use client'

import { createContext, useContext, useState } from 'react'
import type { Session } from './types'

const AuthContext = createContext<Session | null>(null)

interface AuthProvider {
	children: React.ReactNode
	initialSession: Session | null
}

export function AuthProvider({ initialSession, children }: AuthProvider) {
	const [session, _setSession] = useState<Session | null>(initialSession)

	console.log(session, initialSession)

	return <AuthContext.Provider value={session}>{children}</AuthContext.Provider>
}

export function useSession() {
	const session = useContext(AuthContext)

	// if (!session) {
	// 	throw new Error('useSessionContext must be used within a AuthProvider')
	// }

	return session
}
