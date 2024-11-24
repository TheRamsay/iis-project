import classNames from 'classnames'
import Image from 'next/image'
import Link from 'next/link'
import { Avatar } from '../avatar'

type User = {
	id: string
	username: string
	avatar: {
		src: string | undefined
	}
}

interface UserAvatarName {
	user: User
	className?: string
	size?: 'small' | 'full'
	disableLink?: boolean
}

const style = {
	small: 'space-x-1 text-sm',
	full: 'space-x-3 p-2',
}

export function UserAvatarName({
	user,
	className,
	size = 'full',
	disableLink = false,
}: UserAvatarName) {
	const pix = size === 'small' ? 16 : 32

	const _Link = disableLink ? 'div' : Link

	return (
		<div
			className={classNames(
				'flex flex-row items-center w-full',
				style[size],
				className,
			)}
		>
			<_Link href={`/profile/${user.username}`}>
				<Avatar
					name={user.username}
					unoptimized={true}
					src={user.avatar.src}
					alt="avatar"
					size={pix}
					className="rounded-full"
				/>
			</_Link>
			<_Link href={`/profile/${user.username}`}>
				<span>{user.username}</span>
			</_Link>
		</div>
	)
}
