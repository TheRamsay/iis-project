'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { PencilIcon } from 'lucide-react'
import Link from 'next/link'

interface PostEditButton {
	postId: number
	postAuthorId: string
	size?: 'small' | 'full'
}

export function PostEditButton({
	postId,
	postAuthorId,
	size = 'full',
}: PostEditButton) {
	const session = useSession()

	if (!session || session.userId !== postAuthorId) {
		return null
	}

	const pix = size === 'small' ? 16 : 28

	return (
		<Link href={`/post/${postId}/edit`}>
			<PencilIcon
				width={pix}
				height={pix}
				className="cursor-pointer flex-shrink-0"
			/>
		</Link>
	)
}
