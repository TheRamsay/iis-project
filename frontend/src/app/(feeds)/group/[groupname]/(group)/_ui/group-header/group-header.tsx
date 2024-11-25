import { getSession } from '@/app/_lib/auth/get-session'
import { Button } from '@/components/components/button'
import Image from 'next/image'
import Link from 'next/link'
import { GroupHeaderFollow, type GroupType } from './group-header-follow'
import { isMinModerator } from '@/app/_lib/get-permission-level'
import {
	fetchGroupByUsername,
	type Group,
} from '@/app/(feeds)/group/_lib/fetch-groups-by-username'

interface GroupHeader {
	groupname: string
}

export async function GroupHeader({ groupname }: GroupHeader) {
	const group = await fetchGroupByUsername(groupname)

	return (
		<div className="flex w-full justify-between items-center space-x-4">
			<div className="flex-col flex">
				<div className="flex-row flex items-center space-x-6">
					<p className="space-y-2 [word-break:break-word]">
						<span className="text-2xl float-left">{group.groupname}</span>
						{/* <br />
						<span className="text-sm text-gray-300">{group.description}</span> */}
					</p>
				</div>
			</div>
			<GroupActions group={group} />
		</div>
	)
}

interface GroupActions {
	group: Group
}

async function GroupActions({ group }: GroupActions) {
	const session = await getSession()

	const groupType: GroupType = 'private' as GroupType
	const isManager = group.admin.id === session?.userId

	const actions: React.ReactNode[] = []

	if (session && !isManager) {
		actions.push(
			<GroupHeaderFollow
				key="follow"
				groupId={group.id}
				groupType={groupType}
			/>,
		)
	}

	if (isManager) {
		actions.push(
			<Link href={`/group/${group.groupname}/settings`} key="settings">
				<Button variant="outline" fullWidth>
					Settings
				</Button>
			</Link>,
		)
	}

	if (isMinModerator(session?.role)) {
		actions.push(
			<Button variant="destructive" fullWidth>
				Delete
			</Button>,
		)
	}

	return <div className="flex flex-col space-y-2">{actions}</div>
}
