import { Container } from '@/components/components/container'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'
import { Sidebar } from '@/app/_ui/sidebar'
import { getSession } from '@/app/_lib/auth/get-session'

const sidebarItems = (id: string) => [
	{ name: 'Settings', path: `/group/${id}/settings` },
	{ name: 'Requests', path: `/group/${id}/settings/requests` },
	{ name: 'Users', path: `/group/${id}/settings/users` },
]

export default async function Layout({
	children,
	params: { groupname },
}: { children: React.ReactNode; params: { groupname: string } }) {
	const session = await getSession()

	// TODO: Implement isManager etc depending on who we want to allow to access this page
	if (!session) {
		return redirect('/')
	}

	return (
		<Container
			maxWidth="3xl"
			className="flex flex-row justify-between space-x-16 py-8 h-full"
		>
			<Sidebar items={sidebarItems(groupname)} />
			<div className="min-h-full bg-accent w-px" />
			<div className="w-full">{children}</div>
		</Container>
	)
}
