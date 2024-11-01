import { Container } from '@/components/components/container'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'
import { Sidebar } from '../_ui/sidebar'
import { getSession } from '../_lib/auth/get-session'

const sidebarItems = [
	{ name: 'Dashboard', path: '/admin' },
	{ name: 'Users', path: '/admin/users' },
]

export default async function Layout({
	children,
}: { children: React.ReactNode }) {
	const session = await getSession()

	if (!session || session.role === 'regular') {
		return redirect('/')
	}

	return (
		<Container
			maxWidth="3xl"
			className="flex flex-row justify-between space-x-16 py-8 h-full"
		>
			<Sidebar items={sidebarItems} />
			<div className="min-h-full bg-accent w-px" />
			<div className="w-full">{children}</div>
		</Container>
	)
}
