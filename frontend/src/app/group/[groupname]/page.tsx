import { Separator } from '@/components/components/separator'
import { GroupHeader } from './ui/group-header/group-header'
import { MiniatureFeed } from '@/app/_ui/feed/miniature-feed'
import { Suspense } from 'react'
import { getSession } from '@/app/_lib/auth/get-session'

import type {} from 'next/app'

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

	const isMember = await false
	const isPublic = await false

	if (!isMember && !isPublic) {
		return (
			<div className="w-full flex justify-center text-xl">
				This group is private and you are not a member
			</div>
		)
	}

	return <MiniatureFeed groupname={groupname} searchParams={searchParams} />
}
