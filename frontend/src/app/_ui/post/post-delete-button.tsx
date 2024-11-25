'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { isMinModerator } from '@/app/_lib/get-permission-level'
import { useMutation } from '@tanstack/react-query'
import { Trash2Icon } from 'lucide-react'
import { ErrorTooltip } from '../error-tooltip'
import type { Post } from '@/app/post/_lib/fetch-post'
import { backendFetch } from '@/app/_lib/backend-fetch'
import { usePathname, useRouter } from 'next/navigation'

interface PostDeleteButton {
	post: Pick<Post, 'id'> & {
		user: Pick<Post['user'], 'id'>
	}
	group?: {
		id: string
		moderatorId: string
	}
	size?: 'small' | 'full'
}

export function PostDeleteButton({
	post,
	group,
	size = 'full',
}: PostDeleteButton) {
	const session = useSession()

	const router = useRouter()
	const path = usePathname()

	const isGroupModerator = group?.moderatorId === session?.userId

	const { mutate, error } = useMutation({
		mutationKey: ['delete-post', post, group],
		mutationFn: async () => {
			let link = `/api/posts/${post.id}`
			if (isGroupModerator) {
				link = `/api/posts/${post.id}/group/${group?.id}`
			}

			const response = await backendFetch(link, {
				method: 'DELETE',
			})

			if (!response.ok) {
				throw new Error('Failed to delete post')
			}
		},
		onSuccess: () => {
			if (path.startsWith('/post/')) {
				router.push('/')
			} else {
				router.refresh()
			}
		},
	})

	if (!session) {
		return null
	}

	if (
		isMinModerator(session.role) ||
		session.userId === post.user.id ||
		isGroupModerator
	) {
		const pix = size === 'small' ? 16 : 28

		return (
			<div className="flex items-center space-x-2">
				<ErrorTooltip error={error} size={size} />
				<Trash2Icon
					width={pix}
					height={pix}
					color="#9F0000"
					className="cursor-pointer flex-shrink-0"
					onClick={() => mutate()}
				/>
			</div>
		)
	}
}
