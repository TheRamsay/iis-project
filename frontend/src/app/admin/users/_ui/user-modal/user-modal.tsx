'use client'

import type { schema } from '@/app/_lib/db'
import {
	Dialog,
	DialogContent,
	DialogFooter,
	DialogTitle,
	DialogTrigger,
} from '@/components/components/dialog'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useEffect, useState } from 'react'
import { FormProvider, useForm } from 'react-hook-form'
import { UserModalForm } from './user-modal-form'
import { Button } from '@/components/components/button'
import { Loader } from '@/components/components/loader'
import classNames from 'classnames'

type UserModal = {
	children: React.ReactNode
} & Pick<typeof schema.user.$inferSelect, 'id'>

type User = Pick<
	typeof schema.user.$inferSelect,
	'id' | 'displayName' | 'email' | 'isBlocked' | 'userType' | 'username'
> & {
	image: globalThis.File | null
}

export type UserForm = Pick<User, 'id'> & Partial<User>

export function UserModal({ children, id }: UserModal) {
	const [open, setOpen] = useState(false)

	const { data, isFetching, refetch } = useQuery<User>({
		queryKey: ['admin-user', id],
		queryFn: async () => {
			// TODO: Endpoint
			await new Promise((resolve) => setTimeout(resolve, 1000))

			return {
				id: Math.random().toString(),
				displayName: 'John Doe',
				avatarUrl: 'https://example.com/favicon.ico',
				email: 'asdas@goog.eoco',
				username: 'johndoe',
				isBlocked: false,
				userType: 'regular',
				image: null,
			}
		},
		enabled: open,
	})

	const { mutate, isPending } = useMutation({
		mutationKey: ['admin-user', id],
		mutationFn: async (data: UserForm) => {
			// TODO: Endpoint
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onSuccess: () => {
			refetch()
		},
	})

	const loading = isFetching || isPending

	const form = useForm<UserForm>({
		disabled: loading,
		defaultValues: {
			displayName: '',
			email: '',
			isBlocked: false,
			userType: 'regular',
			username: '',
			image: null,
			id: '',
		},
	})

	useEffect(() => {
		if (data) {
			form.reset(data, { keepDirty: false })
		}
	}, [data, form.reset])

	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogTrigger>{children}</DialogTrigger>
			<DialogContent>
				<FormProvider {...form}>
					<DialogTitle>User Settings</DialogTitle>
					<UserModalForm form={form} />
					<DialogFooter>
						<div className="flex flex-row w-full justify-between items-center">
							<div className={classNames(!loading && 'hidden')}>
								<Loader size={20} />
							</div>
							<div className="flex w-full justify-end space-x-4">
								<Button
									onClick={() => mutate(form.watch())}
									disabled={loading || !form.formState.isDirty}
								>
									Save
								</Button>
								<Button onClick={() => setOpen(false)} variant="destructive">
									Cancel
								</Button>
							</div>
						</div>
					</DialogFooter>
				</FormProvider>
			</DialogContent>
		</Dialog>
	)
}
