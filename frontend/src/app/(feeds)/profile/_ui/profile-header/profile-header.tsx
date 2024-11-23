import type { schema } from '@/app/_lib/db'
import { Button } from '@/components/components/button'
import Image from 'next/image'
import Link from 'next/link'
import { ProfileHeaderFollow } from './profile-header-follow'
import { getSession } from '@/app/_lib/auth/get-session'

interface ProfileHeader {
	username: string
}

export async function ProfileHeader({ username }: ProfileHeader) {
	// TODO: endpoint
	const profile = {
		id: 'user_id',
		username: 'johndoe',
		description: 'I am a person.',
		avatar: {
			src: 'https://www.gravatar.com/avatar/',
			width: 32,
			height: 32,
		},
	}

	const session = await getSession()

	let role:
		| 'unregistered'
		| 'owner'
		| (typeof schema.userType.enumValues)[number]
	if (!session) {
		role = 'unregistered'
	} else if (profile.id === session.userId) {
		role = 'owner'
	} else {
		role = session.role
	}

	return (
		<div className="flex w-full justify-between items-center space-x-4">
			<div className="flex-col flex">
				<div className="flex-row flex items-center space-x-6">
					<div className="relative h-28 w-28 rounded-full shrink-0">
						<Image
							unoptimized
							src={profile.avatar.src}
							fill
							alt={profile.username}
							className="rounded-full"
						/>
					</div>
					<p className="space-y-2 [word-break:break-word]">
						<span className="text-2xl float-left">{profile.username}</span>
						<br />
						<span className="text-sm text-gray-300">{profile.description}</span>
					</p>
				</div>
			</div>
			<ProfileActions username={username} role={role} />
		</div>
	)
}

interface ProfileActions {
	username: string
	role: 'unregistered' | 'owner' | (typeof schema.userType.enumValues)[number]
}

function ProfileActions({ username, role }: ProfileActions) {
	const actions: React.ReactNode[] = []

	switch (role) {
		case 'regular': {
			actions.push(<ProfileHeaderFollow key="follow" username={username} />)
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
			actions.push(<ProfileHeaderFollow key="follow" username={username} />)
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
