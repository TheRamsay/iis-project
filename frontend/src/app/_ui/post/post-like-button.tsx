'use client'

import { Heart } from 'lucide-react'
import { useCallback } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import classNames from 'classnames'
import { useSession } from '@/app/_lib/auth/auth-provider'

interface PostLikeButton {
	postId: number
	likeCount: number
}

type LikeData = {
	currentLikes: number
	isLiked: boolean
}

export function PostLikeButton({ postId, likeCount }: PostLikeButton) {
	const session = useSession()

	const queryClient = useQueryClient()

	const { data, refetch } = useQuery<LikeData>({
		queryKey: ['like', session?.userId, postId],
		queryFn: async () => {
			return {
				currentLikes: 0,
				isLiked: false,
			}
		},
		enabled: !!session,
		placeholderData: {
			currentLikes: likeCount,
			isLiked: false,
		},
	})

	const { mutateAsync } = useMutation({
		mutationKey: ['like', postId],
		mutationFn: async () => {},
		onMutate: async () => {
			await queryClient.cancelQueries({
				queryKey: ['like', session?.userId, postId],
			})
			const previous = queryClient.getQueryData<LikeData>([
				'like',
				session?.userId,
				postId,
			])
			queryClient.setQueryData<LikeData>(
				['like', session?.userId, postId],
				(old) => {
					if (old) {
						return {
							currentLikes: old.currentLikes + 1,
							isLiked: !old.isLiked,
						}
					}
					return {
						currentLikes: likeCount + 1,
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
		<div className="space-x-3 flex items-center">
			<Heart
				className={classNames(
					data?.isLiked && 'fill-red-600 text-red-600',
					session && 'cursor-pointer hover:text-accent-foreground',
				)}
				width={28}
				height={28}
				onClick={() => mutateAsync()}
			/>
			<span className="font-semibold">{data?.currentLikes}</span>
		</div>
	)
}
