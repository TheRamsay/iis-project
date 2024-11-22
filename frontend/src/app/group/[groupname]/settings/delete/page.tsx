'use client'

import { Button, Loader } from '@/components/components'
import { useMutation } from '@tanstack/react-query'
import { redirect, useRouter } from 'next/navigation'

export default function Page({ params }: { params: { groupname: string } }) {
	const { push } = useRouter()

	const group = {
		id: '1',
		name: 'Group 1',
	}

	const { mutate, isPending } = useMutation({
		mutationKey: ['delete-group', group.id],
		mutationFn: async () => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onSuccess: () => {
			push('/')
		},
	})

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Delete Group</h1>
			<div className="flex items-center space-x-2">
				<Button variant="destructive" onClick={() => mutate()}>
					Delete Group
				</Button>
				{isPending && <Loader />}
			</div>
		</div>
	)
}
