import { Container } from '@/components/components/container'
import { redirect } from 'next/navigation'
import { Sidebar, type SidebarItem } from '@/app/_ui/sidebar'
import { getSession } from '@/app/_lib/auth/get-session'
import { fetchGroupByUsername } from '../../_lib/fetch-groups-by-username'

const sidebarItems = (id: string): SidebarItem[] => [
	{ name: 'Dashboard', path: `/group/${id}/settings` },
	{ name: 'Requests', path: `/group/${id}/settings/requests` },
	{ name: 'Users', path: `/group/${id}/settings/users` },
	{
		name: 'Delete',
		path: `/group/${id}/settings/delete`,
		className: '!text-red-500',
	},
]

export default async function Layout({
	children,
	params: { groupname },
}: { children: React.ReactNode; params: { groupname: string } }) {
	const session = await getSession()

	if (!session) {
		return redirect('/')
	}

	const group = await fetchGroupByUsername(groupname)

	if (group.admin.id !== session?.userId) {
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
