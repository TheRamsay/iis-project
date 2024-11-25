'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { isMinModerator } from '@/app/_lib/get-permission-level'
import { useMutation } from '@tanstack/react-query'
import { Trash2Icon } from 'lucide-react'
import type { Post } from '@/app/post/_lib/fetch-post'
import type { Comment } from '@/app/_types/comments'
import { ErrorTooltip } from '../../error-tooltip'
import { backendFetch } from '@/app/_lib/backend-fetch'
import { useRouter } from 'next/navigation'

interface PostCommentDeleteButton {
	post: Pick<Post, 'id'>
	comment: Comment
	size?: 'small' | 'full'
}

export function PostCommentDeleteButton({
	post,
	comment,
	size = 'full',
}: PostCommentDeleteButton) {
	const session = useSession()

	const router = useRouter()

	const { mutate, error } = useMutation({
		mutationKey: ['delete-comment', comment],
		mutationFn: async () => {
			const response = await backendFetch(
				`/api/posts/${post.id}/comment/${comment.id}`,
				{
					method: 'DELETE',
				},
			)

			if (!response.ok) {
				throw new Error('Failed to delete post')
			}
		},
		onSuccess: () => {
			router.refresh()
		},
	})

	if (!session) {
		return null
	}

	if (isMinModerator(session.role) || session.userId === comment.user.id) {
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
