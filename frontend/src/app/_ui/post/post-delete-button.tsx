'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { isMinModerator } from '@/app/_lib/get-permission-level'
import { useMutation } from '@tanstack/react-query'
import { Trash2Icon } from 'lucide-react'
import { ErrorTooltip } from '../error-tooltip'
import type { Post } from '@/app/post/_lib/fetch-post'
import { backendFetch } from '@/app/_lib/backend-fetch'

interface PostDeleteButton {
	post: Pick<Post, 'id'> & {
		user: Pick<Post['user'], 'id'>
	}
	groupModeratorId?: string
	size?: 'small' | 'full'
}

export function PostDeleteButton({
	post,
	groupModeratorId,
	size = 'full',
}: PostDeleteButton) {
	const session = useSession()

	const { mutate, error } = useMutation({
		mutationKey: ['delete-post', post, groupModeratorId],
		mutationFn: async () => {
			const response = await backendFetch(`/api/posts/${post.id}`, {
				method: 'DELETE',
				credentials: 'include',
			})

			if (!response.ok) {
				throw new Error('Failed to delete post')
			}

			return response.json()
		},
	})

	if (!session) {
		return null
	}

	if (
		isMinModerator(session.role) ||
		session.userId === post.user.id ||
		groupModeratorId === session.userId
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
