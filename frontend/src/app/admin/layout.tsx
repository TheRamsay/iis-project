import { Container } from '@/components/components/container'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'
import { Sidebar } from '../_ui/sidebar'

const sidebarItems = [
	{ name: 'Dashboard', path: '/admin' },
	{ name: 'Users', path: '/admin/users' },
]

export default function Layout({ children }: { children: React.ReactNode }) {
	const cookiez = cookies()

	const userId = 1231
	const user = {
		isAdmin: true,
	}

	if (!user.isAdmin) {
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
