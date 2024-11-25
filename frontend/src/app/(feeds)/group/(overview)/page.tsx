import { SquareArrowOutUpRight } from 'lucide-react'
import Link from 'next/link'

export default async function Page() {
	const groups = [
		{
			id: 1,
			name: 'Group 1',
		},
		{
			id: 2,
			name: 'Group 2',
		},
	]

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Group Overview</h1>
			<div className="space-y-4">
				<h2 className="text-2xl">My Groups</h2>
				<div className="space-y-2">
					{groups.map((group) => (
						<div
							key={group.id}
							className="justify-between w-full flex items-center"
						>
							<div>{group.name}</div>
							<Link href={`/group/${group.name}`}>
								<SquareArrowOutUpRight size={16} className="text-blue-500" />
							</Link>
						</div>
					))}
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
