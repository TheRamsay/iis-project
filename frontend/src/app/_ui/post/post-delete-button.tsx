'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { isMinModerator } from '@/app/_lib/get-permission-level'
import { useMutation } from '@tanstack/react-query'
import { Trash2Icon } from 'lucide-react'
import { ErrorTooltip } from '../error-tooltip'

interface PostDeleteButton {
	postId: number
	postAuthorId: string
	size?: 'small' | 'full'
}

export function PostDeleteButton({
	postId,
	postAuthorId,
	size = 'full',
}: PostDeleteButton) {
	const session = useSession()

	const { mutate, error } = useMutation({
		mutationKey: ['delete-post', postId],
		mutationFn: async () => {
			// TODO: endpoint
		},
	})

	if (!session) {
		return null
	}

	if (isMinModerator(session.role) || session.userId === postAuthorId) {
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
