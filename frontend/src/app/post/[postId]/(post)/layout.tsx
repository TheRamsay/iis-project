import { Container } from '@/components/components/container'
import type React from 'react'

export default async function Layout({
	children,
}: { children: React.ReactNode }) {
	return (
		<Container maxWidth="7xl" className="py-8 pb-40 h-full">
			{children}
		</Container>
	)
}
