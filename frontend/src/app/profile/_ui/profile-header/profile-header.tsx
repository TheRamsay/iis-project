import type { schema } from '@/app/_lib/db'
import { Button } from '@/components/components/button'
import Image from 'next/image'
import Link from 'next/link'
import { ProfileHeaderFollow } from './profile-header-follow'

interface ProfileHeader {
	profileId: string
}

export function ProfileHeader({ profileId }: ProfileHeader) {
	const profile = {
		id: 'user_id',
		username: 'johndoe',
		bio: 'I am a person.',
		avatar: {
			src: 'https://www.gravatar.com/avatar/',
			width: 32,
			height: 32,
		},
	}

	const loggedInUser = {
		id: 'user_ida',
		role: 'moderator',
	} as const

	let role:
		| 'unregistered'
		| 'owner'
		| (typeof schema.userType.enumValues)[number]
	if (!loggedInUser) {
		role = 'unregistered'
	} else if (profile.id === loggedInUser.id) {
		role = 'owner'
	} else {
		role = loggedInUser.role
	}

	return (
		<div className="flex w-full justify-between items-center">
			<div className="flex-col flex">
				<div className="flex-row flex items-center space-x-6">
					<div className="relative h-28 w-28 rounded-full">
						<Image
							unoptimized
							src={profile.avatar.src}
							fill
							alt={profile.username}
							className="rounded-full"
						/>
					</div>
					<div>
						<span className="text-2xl">{profile.username}</span>
					</div>
				</div>
			</div>
			<ProfileActions profileId={profileId} role={role} />
		</div>
	)
}

interface ProfileActions {
	profileId: string
	role: 'unregistered' | 'owner' | (typeof schema.userType.enumValues)[number]
}

function ProfileActions({ profileId, role }: ProfileActions) {
	const actions: React.ReactNode[] = []

	switch (role) {
		case 'regular': {
			actions.push(<ProfileHeaderFollow key="follow" profileId={profileId} />)
			break
		}
		case 'owner': {
			actions.push(
				<Link href="/settings">
					<Button key="edit" variant="outline">
						Edit Profile
					</Button>
				</Link>,
			)
			break
		}
		case 'moderator':
		case 'administrator':
			actions.push(<ProfileHeaderFollow key="follow" profileId={profileId} />)
			actions.push(
				<Link href="/admin/users">
					<Button key="edit" variant="outline">
						Edit Profile
					</Button>
				</Link>,
			)
	}

	return <div className="flex flex-col space-y-2">{actions}</div>
}
