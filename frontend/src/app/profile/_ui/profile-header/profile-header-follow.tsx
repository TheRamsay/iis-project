'use client'

import { SkeletonText } from '@/components'
import { Button } from '@/components/components/button'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'

interface ProfileHeaderFollow {
	profileId: string
}

export function ProfileHeaderFollow({ profileId }: ProfileHeaderFollow) {
	const loggedInUser = {
		id: 'user_ida',
	}

	const queryClient = useQueryClient()

	const { data, isLoading, refetch } = useQuery({
		queryKey: ['profile-follow', profileId, loggedInUser.id],
		queryFn: async () => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
			return {
				isFollowing: false,
			}
		},
	})

	const { mutate } = useMutation<void, void, boolean>({
		mutationKey: ['profile-follow', profileId, loggedInUser.id],
		mutationFn: async (follow) => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onMutate: async () => {
			await queryClient.cancelQueries({
				queryKey: ['profile-follow', profileId, loggedInUser.id],
			})

			queryClient.setQueryData(
				['profile-follow', profileId, loggedInUser.id],
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
