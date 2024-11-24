'use client'

import { Loader } from '@/components/components/loader'
import { Button } from '@/components/components/button'
import { TextField } from '@/components/components/text-field'
import { useMutation } from '@tanstack/react-query'
import { useState } from 'react'
import { ErrorTooltip } from '../../error-tooltip'

interface PostCommentAdd {
	postId: number
}

export function PostCommentAdd({ postId }: PostCommentAdd) {
	const [comment, setComment] = useState('')

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['add-comment', postId],
		mutationFn: async () => {
			const response = await fetch(`/api/posts/${postId}/comment`, {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify({ content: comment }),
			})

			if (!response.ok) {
				throw new Error('Failed to add comment')
			}

			return response.json()
		},
		onSuccess: () => {
			setComment('')
		},
	})

	const disabled = comment.trim().length === 0

	return (
		<div className="flex justify-between w-full">
			<TextField
				type="text"
				variant="naked"
				value={comment}
				onValueChange={setComment}
				placeholder="Add a comment..."
				className=""
				size="sm"
			/>
			<div className="flex flex-row space-x-2 items-center">
				{isPending && <Loader />}
				<ErrorTooltip error={error} />
				<Button variant="ghost" disabled={disabled} onClick={() => mutate()}>
					Post
				</Button>
			</div>
		</div>
	)
}
