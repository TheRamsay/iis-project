import { Suspense } from 'react'
import { ProfileHeader } from './profile-header/profile-header'
import { ProfileFeed } from './profile-feed'
import { Separator } from '@/components/components/separator'

interface ProfilePage {
	username: string
}

export function ProfilePage({ username }: ProfilePage) {
	return (
		<Suspense fallback={null}>
			<_ProfilePage username={username} />
		</Suspense>
	)
}

function _ProfilePage({ username }: ProfilePage) {
	return (
		<div className="space-y-4">
			<ProfileHeader username={username} />
			<Separator orientation="horizontal" className="!bg-accent" />
			<ProfileFeed username={username} />
		</div>
	)
}
