import { Container } from '@/components/components/container'
import { redirect } from 'next/navigation'
import type React from 'react'
import { getSession } from '../_lib/auth/get-session'

export default async function Layout({
	children,
}: { children: React.ReactNode }) {
	const session = await getSession()

	if (!session) {
		return redirect('/login')
	}

	return (
		<Container maxWidth="md" className="pt-4">
			{children}
		</Container>
	)
}
