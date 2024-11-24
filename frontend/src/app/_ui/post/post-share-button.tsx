'use client'

import type { Post } from '@/app/_types/post'
import { Share2 } from 'lucide-react'
import { useRouter } from 'next/navigation'
import { useMemo } from 'react'

type PostShareButton = { post: Pick<Post, 'id'> }

export function PostShareButton({ post }: PostShareButton) {
	const link = useMemo(() => {
		if (typeof location === 'undefined') return ''

		return `${location.href}/post/${post.id}`
	}, [post.id])

	return (
		<Share2
			className="cursor-pointer"
			onClick={() => {
				navigator.clipboard.writeText(link)
			}}
			width={28}
			height={28}
		/>
	)
}
