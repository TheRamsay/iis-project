import { SquareArrowOutUpRight } from 'lucide-react'
import Link from 'next/link'
import { fetchGroupsByUsername } from '../_lib/fetch-groups-by-username'
import { getSession } from '@/app/_lib/auth/get-session'
import { redirect } from 'next/navigation'

export default async function Page() {
	const session = await getSession()

	if (!session) {
		redirect('/login')
	}

	const groups = await fetchGroupsByUsername('', session.userId)

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Group Overview</h1>
			<div className="space-y-4">
				<h2 className="text-2xl">My Groups</h2>
				<div className="space-y-2">
					{groups.length ? (
						groups.map((group) => (
							<div
								key={group.id}
								className="justify-between w-full flex items-center"
							>
								<div>{group.groupname}</div>
								<Link href={`/group/${group.groupname}`}>
									<SquareArrowOutUpRight size={16} className="text-blue-500" />
								</Link>
							</div>
						))
					) : (
						<div>No groups found.</div>
					)}
				</div>
			</div>
			<div className="justify-between w-full flex items-center">
				<h2 className="text-2xl">Create Group</h2>
				<Link href="/group/create">
					<SquareArrowOutUpRight size={24} className="text-blue-500" />
				</Link>
			</div>
		</div>
	)
}
