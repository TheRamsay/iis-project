import { Separator } from '@/components/components/separator'
import { GroupHeader } from './_ui/group-header/group-header'
import { MiniatureFeed } from '@/app/(feeds)/_ui/feed/miniature-feed'
import { Suspense } from 'react'
import { getSession } from '@/app/_lib/auth/get-session'

import { fetchGroupStatus } from '../../_lib/fetch-group-status'
import { fetchGroupByUsername } from '../../_lib/fetch-groups-by-username'

export default function Page({
	params: { groupname },
	searchParams,
}: { params: { groupname: string }; searchParams: Record<string, string> }) {
	return (
		<div className="space-y-4">
			<GroupHeader groupname={groupname} />
			<Separator orientation="horizontal" className="!bg-accent" />
			<Suspense>
				<Feed groupname={groupname} searchParams={searchParams} />
			</Suspense>
		</div>
	)
}

async function Feed({
	groupname,
	searchParams,
}: { groupname: string; searchParams: Record<string, string> }) {
	const session = await getSession()

	const isPublic = false

	if (!session && !isPublic) {
		return (
			<div className="w-full flex justify-center text-xl">
				You must be logged in to view groups
			</div>
		)
	}

	const group = await fetchGroupByUsername(groupname)
	const { status } = await fetchGroupStatus({ groupId: group.id })

	if (status !== 'joined' && !isPublic && group.admin.id !== session?.userId) {
		return (
			<div className="w-full flex justify-center text-xl">
				You are not a member of this group
			</div>
		)
	}

	return <MiniatureFeed groupname={groupname} searchParams={searchParams} />
}
