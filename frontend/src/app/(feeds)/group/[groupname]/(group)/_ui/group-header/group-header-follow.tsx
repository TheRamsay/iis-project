'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { SkeletonText } from '@/components/components/skeleton'
import { Button } from '@/components/components/button'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { ErrorTooltip } from '@/app/_ui/error-tooltip'
import { fetchGroupStatus } from '@/app/(feeds)/group/_lib/fetch-group-status'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'

export enum GroupFollowState {
	NotJoined = 'notJoined',
	Pending = 'pending',
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

		return GroupFollowState.Pending
	}

	return GroupFollowState.NotJoined
}

interface GroupHeaderFollow {
	groupId: string
	groupType: GroupType
}

export function GroupHeaderFollow({ groupId, groupType }: GroupHeaderFollow) {
	const session = useSession()

	const queryClient = useQueryClient()

	const { data, isLoading, refetch } = useQuery({
		queryKey: ['group-follow', groupId, session?.userId],
		queryFn: async () => {
			const data = await fetchGroupStatus({ groupId })

			return {
				followState: data.status,
			}
		},
	})

	console.log(data)

	const { mutate, error } = useMutation<void, Error, boolean>({
		mutationKey: ['group-follow', groupId, session?.userId],
		mutationFn: async (join) => {
			const response = await backendFetch(
				`/api/groups/${groupId}/${join ? 'join' : 'leave'}`,
				{
					method: 'POST',
				},
			)

			await checkResponse(response)
		},
		onMutate: async () => {
			if (!data?.followState) {
				return
			}

			await queryClient.cancelQueries({
				queryKey: ['group-follow', groupId, session?.userId],
			})

			queryClient.setQueryData(
				['group-follow', groupId, session?.userId],
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
			<ErrorShell error={error}>
				<Button variant="outline" fullWidth>
					<SkeletonText />
				</Button>
			</ErrorShell>
		)
	}

	if (data.followState === GroupFollowState.Joined) {
		return (
			<ErrorShell error={error}>
				<Button variant="outline" onClick={() => mutate(false)}>
					Leave
				</Button>
			</ErrorShell>
		)
	}

	if (data.followState === GroupFollowState.Pending) {
		return (
			<ErrorShell error={error}>
				<Button variant="outline" onClick={() => mutate(false)} disabled>
					Cancel
				</Button>
			</ErrorShell>
		)
	}

	if (data.followState === GroupFollowState.NotJoined) {
		if (groupType === GroupType.Public) {
			return (
				<ErrorShell error={error}>
					<Button variant="outline" onClick={() => mutate(true)}>
						Join
					</Button>
				</ErrorShell>
			)
		}

		if (groupType === GroupType.Private) {
			return (
				<ErrorShell error={error}>
					<Button variant="outline" onClick={() => mutate(true)}>
						Request
					</Button>
				</ErrorShell>
			)
		}
	}
}

function ErrorShell({
	error,
	children,
}: { error: Error | null; children: React.ReactNode }) {
	return (
		<div className="flex items-center space-x-2 w-full">
			<ErrorTooltip error={error} />
			{children}
		</div>
	)
}
