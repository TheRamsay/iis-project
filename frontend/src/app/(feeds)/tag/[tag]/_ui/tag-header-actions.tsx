'use client'

import { useSession } from '@/app/_lib/auth/auth-provider'
import { isMinModerator } from '@/app/_lib/get-permission-level'
import { ErrorTooltip } from '@/app/_ui/error-tooltip'
import { Button, Loader } from '@/components/components'
import { useMutation } from '@tanstack/react-query'
import { useRouter } from 'next/navigation'

export function TagHeaderActions() {
	const session = useSession()

	const { push } = useRouter()

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['tag-hide', session?.userId],
		mutationFn: async () => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onSuccess: () => {
			push('/')
		},
	})

	if (!session || !isMinModerator(session.role)) {
		return null
	}

	return (
		<div className="items-center flex space-x-2">
			{isPending && <Loader />}
			<ErrorTooltip error={error} />
			<Button variant="destructive" onClick={() => mutate()}>
				Hide
			</Button>
		</div>
	)
}
