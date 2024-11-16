'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { SkeletonText } from '@/components'
import { Button } from '@/components/components/button'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'

enum GroupFollowState {
	NotJoined = 'not-joined',
	Requested = 'requested',
	Joined = 'joined',
}

export enum GroupType {
	Public = 'public',
	Private = 'private',
}

function changeState(currentState: GroupFollowState, groupType: GroupType) {
	if (currentState === GroupFollowState.NotJoined) {
		if (groupType === GroupType.Public) {
			return GroupFollowState.Joined
		}

		return GroupFollowState.Requested
	}

	return GroupFollowState.NotJoined
}

interface GroupHeaderFollow {
	groupname: string
	groupType: GroupType
}

export function GroupHeaderFollow({ groupname, groupType }: GroupHeaderFollow) {
	const session = useSession()

	const queryClient = useQueryClient()

	const { data, isLoading, refetch } = useQuery({
		queryKey: ['group-follow', groupname, session?.userId],
		queryFn: async () => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
			return {
				followState: GroupFollowState.NotJoined,
			}
		},
	})

	const { mutate } = useMutation<void, void, boolean>({
		mutationKey: ['group-follow', groupname, session?.userId],
		mutationFn: async (follow) => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onMutate: async () => {
			if (!data?.followState) {
				return
			}

			await queryClient.cancelQueries({
				queryKey: ['group-follow', groupname, session?.userId],
			})

			queryClient.setQueryData(
				['group-follow', groupname, session?.userId],
				(old: { followState: GroupFollowState }) => {
					return {
						followState: changeState(old.followState, groupType),
					}
				},
			)

			return { followState: changeState(data.followState, groupType) }
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

	if (data.followState === GroupFollowState.Joined) {
		return (
			<Button variant="outline" onClick={() => mutate(false)}>
				Leave
			</Button>
		)
	}

	if (data.followState === GroupFollowState.Requested) {
		return (
			<Button variant="outline" onClick={() => mutate(false)}>
				Cancel
			</Button>
		)
	}

	if (data.followState === GroupFollowState.NotJoined) {
		if (groupType === GroupType.Public) {
			return (
				<Button variant="outline" onClick={() => mutate(true)}>
					Join
				</Button>
			)
		}

		if (groupType === GroupType.Private) {
			return (
				<Button variant="outline" onClick={() => mutate(true)}>
					Request
				</Button>
			)
		}
	}
}
