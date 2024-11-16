'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { SkeletonText } from '@/components/components/skeleton'
import { Button } from '@/components/components/button'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'

interface ProfileHeaderFollow {
	username: string
}

export function ProfileHeaderFollow({ username }: ProfileHeaderFollow) {
	const session = useSession()

	const queryClient = useQueryClient()

	const { data, isLoading, refetch } = useQuery({
		queryKey: ['profile-follow', username, session?.userId],
		queryFn: async () => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
			return {
				isFollowing: false,
			}
		},
	})

	const { mutate } = useMutation<void, void, boolean>({
		mutationKey: ['profile-follow', username, session?.userId],
		mutationFn: async (follow) => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onMutate: async () => {
			await queryClient.cancelQueries({
				queryKey: ['profile-follow', username, session?.userId],
			})

			queryClient.setQueryData(
				['profile-follow', username, session?.userId],
				(old: { isFollowing: boolean }) => {
					return {
						isFollowing: !old?.isFollowing,
					}
				},
			)

			return { isFollowing: !data?.isFollowing }
		},
		onSettled: () => {
			refetch()
		},
	})

	if (isLoading || !data) {
		return (
			<Button variant="outline">
				<SkeletonText />
			</Button>
		)
	}

	if (data.isFollowing) {
		return (
			<Button variant="outline" onClick={() => mutate(false)}>
				Unfollow
			</Button>
		)
	}

	return (
		<Button variant="outline" onClick={() => mutate(true)}>
			Follow
		</Button>
	)
}
