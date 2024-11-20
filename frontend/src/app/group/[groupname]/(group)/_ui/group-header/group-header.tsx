import { getSession } from '@/app/_lib/auth/get-session'
import { Button } from '@/components/components/button'
import Image from 'next/image'
import Link from 'next/link'
import { GroupHeaderFollow, type GroupType } from './group-header-follow'
import { isMinModerator } from '@/app/_lib/get-permission-level'

interface GroupHeader {
	groupname: string
}

export async function GroupHeader({ groupname }: GroupHeader) {
	const group = {
		name: 'groupname',
		description:
			'descriptiondondescriptiondescriptiondescriptiondescriptiondescriptiondescriptiondescriptiondescription',
		avatar: {
			src: 'https://www.gravatar.com/avatar/',
			width: 32,
			height: 32,
		},
	}

	return (
		<div className="flex w-full justify-between items-center space-x-4">
			<div className="flex-col flex">
				<div className="flex-row flex items-center space-x-6">
					<div className="relative h-28 w-28 rounded-full shrink-0">
						<Image
							unoptimized
							src={group.avatar.src}
							fill
							alt={group.name}
							className="rounded-full"
						/>
					</div>
					<p className="space-y-2 [word-break:break-word]">
						<span className="text-2xl float-left">{group.name}</span>
						<br />
						<span className="text-sm text-gray-300">{group.description}</span>
					</p>
				</div>
			</div>
			<GroupActions groupname={groupname} />
		</div>
	)
}

interface GroupActions {
	groupname: string
}

async function GroupActions({ groupname }: GroupActions) {
	const session = await getSession()

	const groupType: GroupType = 'private' as GroupType
	const isManager = true

	const actions: React.ReactNode[] = []

	if (session) {
		actions.push(
			<GroupHeaderFollow
				key="follow"
				groupname={groupname}
				groupType={groupType}
			/>,
		)
	}

	if (isManager) {
		actions.push(
			<Link href={`/group/${groupname}/settings`} key="settings">
				<Button variant="outline">Settings</Button>
			</Link>,
		)
	}

	if (isMinModerator(session?.role)) {
		actions.push(<Button variant="destructive">Delete</Button>)
	}

	return <div className="flex flex-col space-y-2">{actions}</div>
}
