import { Suspense } from 'react'
import { ProfileHeader } from './profile-header/profile-header'
import { MiniatureFeed } from '../../../_ui/feed/miniature-feed'
import { Separator } from '@/components/components/separator'

interface ProfilePage {
	username: string
	searchParams: Record<string, string>
}

export function ProfilePage({ username, searchParams }: ProfilePage) {
	return (
		<Suspense fallback={null}>
			<_ProfilePage username={username} searchParams={searchParams} />
		</Suspense>
	)
}

function _ProfilePage({ username, searchParams }: ProfilePage) {
	return (
		<div className="space-y-4">
			<ProfileHeader username={username} />
			<Separator orientation="horizontal" className="!bg-accent" />
			<MiniatureFeed username={username} searchParams={searchParams} />
		</div>
	)
}
