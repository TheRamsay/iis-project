'use client'

import { Heart } from 'lucide-react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import classNames from 'classnames'
import { useSession } from '@/app/_lib/auth/auth-provider'
import { ErrorTooltip } from '../error-tooltip'
import { backendFetch } from '@/app/_lib/backend-fetch'
import type { Post } from '@/app/post/_lib/fetch-post'

type PostLikeButton = { post: Pick<Post, 'id' | 'likeCount'> }

type LikeData = {
	currentLikes: number
	isLiked: boolean
}

export function PostLikeButton({ post }: PostLikeButton) {
	const session = useSession()

	const queryClient = useQueryClient()

	const { data, refetch } = useQuery<LikeData>({
		queryKey: ['like', session?.userId, post.id],
		queryFn: async () => {
			// TODO: endpoint
			return {
				currentLikes: 0,
				isLiked: false,
			}
		},
		enabled: !!session,
		placeholderData: {
			currentLikes: post.likeCount,
			isLiked: false,
		},
	})

	const { mutate, error } = useMutation({
		mutationKey: ['like', post.id],
		mutationFn: async () => {
			const response = await backendFetch(`/api/posts/${post.id}/like`, {
				method: data?.isLiked ? 'DELETE' : 'POST',
			})

			if (!response.ok) {
				throw new Error('Failed to like post')
			}

			return response.json()
		},
		onMutate: async () => {
			await queryClient.cancelQueries({
				queryKey: ['like', session?.userId, post.id],
			})
			const previous = queryClient.getQueryData<LikeData>([
				'like',
				session?.userId,
				post.id,
			])
			queryClient.setQueryData<LikeData>(
				['like', session?.userId, post.id],
				(old) => {
					if (old) {
						return {
							currentLikes: old.currentLikes + 1,
							isLiked: !old.isLiked,
						}
					}
					return {
						currentLikes: post.likeCount + 1,
						isLiked: true,
					}
				},
			)

			return { previous }
		},
		onSettled: () => {
			refetch()
		},
	})

	return (
		<div className="space-x-3 items-center flex">
			<ErrorTooltip error={error} />
			<div className="space-x-3 flex items-center">
				<Heart
					className={classNames(
						data?.isLiked && 'fill-red-600 text-red-600',
						session && 'cursor-pointer hover:text-accent-foreground',
					)}
					width={28}
					height={28}
					onClick={() => mutate()}
				/>
				<span className="font-semibold">{data?.currentLikes}</span>
			</div>
		</div>
	)
}
