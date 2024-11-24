'use client'

import { Loader } from '@/components/components/loader'
import { Button } from '@/components/components/button'
import { useMutation } from '@tanstack/react-query'
import { useRouter } from 'next/navigation'
import { ErrorTooltip } from '@/app/_ui/error-tooltip'
import type { Group } from '@/app/(feeds)/group/_lib/fetch-groups-by-username'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'

interface GroupDelete {
	group: Group
}

export function GroupDelete({ group }: GroupDelete) {
	const { push } = useRouter()

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['delete-group', group.id],
		mutationFn: async () => {
			const response = await backendFetch(`/api/groups/${group.id}`, {
				method: 'DELETE',
			})

			await checkResponse(response)

			return response.json()
		},
		onSuccess: () => {
			push('/')
		},
	})

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Delete Group</h1>
			<div className="flex items-center space-x-4">
				<Button variant="destructive" onClick={() => mutate()}>
					Delete Group
				</Button>
				{error && <ErrorTooltip error={error} />}
				{isPending && <Loader />}
			</div>
		</div>
	)
}
