'use client'

import { ErrorTooltip } from '@/app/_ui/error-tooltip'
import { Button, Loader } from '@/components/components'
import { useMutation } from '@tanstack/react-query'
import { redirect, useRouter } from 'next/navigation'

export default function Page({ params }: { params: { groupname: string } }) {
	const { push } = useRouter()

	// TODO: endpoint
	const group = {
		id: '1',
		name: 'Group 1',
	}

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['delete-group', group.id],
		mutationFn: async () => {
			// TODO: endpoint
			await new Promise((resolve) => setTimeout(resolve, 1000))

			throw new Error('Failed to delete group')
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
