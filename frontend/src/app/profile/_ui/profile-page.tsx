import { Suspense } from 'react'
import { ProfileHeader } from './profile-header/profile-header'
import { ProfileFeed } from './profile-feed'
import { Separator } from '@/components/components/separator'

interface ProfilePage {
	profileId: string
}

export function ProfilePage({ profileId }: ProfilePage) {
	return (
		<Suspense fallback={null}>
			<_ProfilePage profileId={profileId} />
		</Suspense>
	)
}

function _ProfilePage({ profileId }: ProfilePage) {
	return (
		<div className="space-y-4">
			<ProfileHeader profileId={profileId} />
			<Separator orientation="horizontal" className="!bg-accent" />
			<ProfileFeed profileId={profileId} />
		</div>
	)
}
