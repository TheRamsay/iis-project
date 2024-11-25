'use client'

import type { schema } from '@/app/_lib/db'
import {
	Dialog,
	DialogContent,
	DialogFooter,
	DialogTitle,
	DialogTrigger,
} from '@/components/components/dialog'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useEffect, useState } from 'react'
import { FormProvider, useForm } from 'react-hook-form'
import { UserModalForm } from './user-modal-form'
import { Button } from '@/components/components/button'
import { Loader } from '@/components/components/loader'
import classNames from 'classnames'
import { FormServerError } from '@/app/_ui/form/form-server-error'
import { fetchUserByUsername } from '@/app/_lib/user/fetch-user'
import type { Role } from '@/app/_types/user'
import { z } from 'zod'
import { myz } from '@/app/_types/zod'
import { formImageSchema } from '@/app/_ui/form/form-image'
import { zodResolver } from '@hookform/resolvers/zod'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'
import { uploadImage } from '@/app/_lib/upload-image'
import { useRouter } from 'next/navigation'
import { ErrorTooltip } from '@/app/_ui/error-tooltip'
import { extractError } from '@/app/_lib/extract-error'
import { useSession } from '@/app/_lib/auth/auth-provider'
import { isMinAdministrator } from '@/app/_lib/get-permission-level'

const userModalSchema = z
	.object({
		id: z.string(),
		description: myz.description,
		email: z.string().email(),
		isBlocked: z.boolean(),
		role: z.string(),
		username: myz.username,
	})
	.merge(formImageSchema(false))

type UserModal = {
	children?: React.ReactNode
	open?: boolean
} & Pick<typeof schema.user.$inferSelect, 'username'>

export type UserForm = Pick<
	typeof schema.user.$inferSelect,
	'id' | 'email' | 'isBlocked' | 'username'
> & {
	description: string
	image: string | null | undefined
	role: Role
}

export function UserModal({
	children,
	username: _username,
	open: _open,
}: UserModal) {
	const [open, setOpen] = useState(_open)
	const [username, setUsername] = useState(_username)

	const session = useSession()

	const queryClient = useQueryClient()

	const { data, isFetching, refetch } = useQuery<UserForm>({
		queryKey: ['admin-user', username],
		queryFn: async () => {
			const user = await fetchUserByUsername(username)

			return {
				...user,
				image: user.avatar.src,
			}
		},
		enabled: !!open,
	})

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['admin-user', username],
		mutationFn: async (formData: UserForm) => {
			if (isMinAdministrator(session?.role)) {
				let imageUrl = formData.image
				if (imageUrl?.startsWith('blob:')) {
					const { link } = await uploadImage(imageUrl)
					imageUrl = link
				}

				const response = await backendFetch(`/api/users/id/${data?.id}`, {
					method: 'PUT',
					body: JSON.stringify({
						description: formData.description,
						username: formData.username,
						email: formData.email,
						avatar_url: imageUrl || undefined,
						user_type: formData.role || 'regular',
					}),
				})

				try {
					await checkResponse(response, { passError: true })
				} catch (error) {
					if (error instanceof Error) {
						throw new Error(extractError(error.message))
					}
					throw error
				}
			}

			if (formData.isBlocked !== data?.isBlocked) {
				const response = await backendFetch(
					`/api/users/id/${data?.id}/${formData.isBlocked ? 'block' : 'unblock'}`,
					{
						method: 'POST',
					},
				)

				await checkResponse(response, {
					customError: 'Failed to block user',
				})
			}

			return { username: formData.username }
		},
		onSuccess: (data) => {
			if (username !== data.username) {
				queryClient.invalidateQueries({ queryKey: ['admin-users'] })
			}

			setUsername(data.username)
			refetch()
		},
	})

	const loading = isFetching || isPending

	const form = useForm<UserForm>({
		disabled: loading,
		defaultValues: {
			description: '',
			email: '',
			isBlocked: false,
			role: 'regular',
			username: username,
			image: null,
			id: '',
		},
		resolver: zodResolver(userModalSchema),
	})

	useEffect(() => {
		if (data) {
			form.reset(data, { keepDirty: false })
		}
	}, [data, form.reset])

	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogTrigger>{children}</DialogTrigger>
			<DialogContent className="overflow-y-auto max-h-full">
				<FormProvider {...form}>
					<DialogTitle>User Settings</DialogTitle>
					<FormServerError error={error} />
					<UserModalForm form={form} />
					<DialogFooter>
						<div className="flex flex-row w-full justify-between items-center">
							<div className={classNames(!loading && 'hidden')}>
								<Loader size={20} />
							</div>
							<div className="flex w-full justify-end space-x-4">
								<PostDeleteButton userId={data?.id} />
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

function PostDeleteButton({ userId }: { userId: string | undefined }) {
	const router = useRouter()

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['delete-user', userId],
		mutationFn: async () => {
			const response = await backendFetch(`/api/users/id/${userId}`, {
				method: 'DELETE',
			})

			if (!response.ok) {
				throw new Error('Failed to delete user')
			}
		},
		onSuccess: () => {
			router.refresh()
		},
	})

	return (
		<div className="flex items-center space-x-2">
			<div className={classNames(!isPending && 'hidden')}>
				<Loader size={20} />
			</div>
			<ErrorTooltip error={error} size="small" />
			<Button onClick={() => mutate()} variant="destructive">
				Delete User
			</Button>
		</div>
	)
}
