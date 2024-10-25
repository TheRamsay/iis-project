import { Container } from '@/components/components/container'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'
import { Sidebar } from '../_ui/sidebar'

const sidebarItems = [
	{
		name: 'Settings',
		path: '/settings',
	},
	{
		name: 'Log Out',
		path: '/logout',
		className: '!text-red-500',
	},
]

export default function Layout({ children }: { children: React.ReactNode }) {
	const cookiez = cookies()

	const user_id = 1231

	if (!user_id) {
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
