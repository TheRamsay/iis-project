import { getSession } from '@/app/_lib/auth/get-session'
import { Container } from '@/components/components/container'
import { redirect } from 'next/navigation'
import type React from 'react'

export default async function Layout({
	children,
}: { children: React.ReactNode }) {
	const session = await getSession()

	if (session) {
		redirect('/')
	}

	return (
		<Container maxWidth="md" className="pt-4">
			{children}
		</Container>
	)
}
