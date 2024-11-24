'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { SkeletonText } from '@/components/components/skeleton'
import { Button } from '@/components/components/button'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { ErrorTooltip } from '@/app/_ui/error-tooltip'

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
			// TODO: endpoint
			await new Promise((resolve) => setTimeout(resolve, 1000))
			return {
				followState: GroupFollowState.NotJoined,
			}
		},
	})

	const { mutate, error } = useMutation<void, Error, boolean>({
		mutationKey: ['group-follow', groupname, session?.userId],
		mutationFn: async (follow) => {
			// TODO: endpoint
			await new Promise((resolve) => setTimeout(resolve, 1000))
			throw new Error('Failed to follow group')
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

	if (data.followState === GroupFollowState.Requested) {
		return (
			<ErrorShell error={error}>
				<Button variant="outline" onClick={() => mutate(false)}>
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
