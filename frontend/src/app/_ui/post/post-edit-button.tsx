'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import type { Post } from '@/app/post/_lib/fetch-post'
import { PencilIcon } from 'lucide-react'
import Link from 'next/link'

interface PostEditButton {
	post: Pick<Post, 'id'> & {
		user: Pick<Post['user'], 'id'>
	}
	size?: 'small' | 'full'
}

export function PostEditButton({ post, size = 'full' }: PostEditButton) {
	const session = useSession()

	if (!session || session.userId !== post.user.id) {
		return null
	}

	const pix = size === 'small' ? 16 : 28

	return (
		<Link href={`/post/${post.id}/edit`}>
			<PencilIcon
				width={pix}
				height={pix}
				className="cursor-pointer flex-shrink-0"
			/>
		</Link>
	)
}
