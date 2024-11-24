'use client'

import type { schema } from '@/app/_lib/db'
import { formClassnames } from '@/app/_lib/form-classnames'
import { TextField } from '@/components/components/text-field'
import { FormControl, FormField, FormItem } from '@/components/components/form'
import { Controller, FormProvider, useForm } from 'react-hook-form'
import { Button } from '@/components/components/button'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useEffect } from 'react'
import { Loader } from '@/components/components/loader'
import classNames from 'classnames'
import { FormImage, formImageSchema } from '@/app/_ui/form/form-image'
import { FormLabelError } from '@/app/_ui/form/form-label-error'
import { TextArea } from '@/components/components'
import { FormServerError } from '@/app/_ui/form/form-server-error'
import { fetchUserById } from '@/app/_lib/user/fetch-user'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'
import { uploadImage } from '@/app/_lib/upload-image'
import { z, type ZodType } from 'zod'
import { myz } from '@/app/_types/zod'
import { zodResolver } from '@hookform/resolvers/zod'

// TODO: validation

const userSchema: ZodType<User> = z
	.object({
		id: z.string(),
		displayName: myz.displayName,
		email: z.string().email(),
		username: myz.username,
		description: myz.description,
		password: myz.password.or(z.literal('')),
	})
	.merge(formImageSchema(false))

interface UserFormProps {
	userId: string
}

type User = Pick<
	typeof schema.user.$inferSelect,
	'id' | 'displayName' | 'email' | 'username'
> & { image: string | null; description: string; password: string }

export type UserForm = Pick<User, 'id'> & Partial<User>

export function UserForm({ userId }: UserFormProps) {
	const { data, isFetching, refetch } = useQuery({
		queryKey: ['edit-user', userId],
		queryFn: async () => {
			const user = await fetchUserById(userId)

			return {
				...user,
				password: '',
				image: user.avatar.src,
			}
		},
	})

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['edit-user', userId],
		mutationFn: async (formData: UserForm) => {
			let imageUrl = formData.image
			if (imageUrl?.startsWith('blob:')) {
				const { link } = await uploadImage(imageUrl)
				imageUrl = link
			}

			const response = await backendFetch(`/api/users/id/${userId}`, {
				method: 'PUT',
				body: JSON.stringify({
					display_name: formData.displayName,
					username: formData.username,
					email: formData.email,
					avatar_url: imageUrl || undefined,
					user_type: data?.role || 'regular',
				}),
			})

			await checkResponse(response, 'Failed to update user')

			return response.json()
		},
		onSuccess: () => {
			refetch()
		},
	})

	const loading = isFetching || isPending

	const form = useForm<UserForm>({
		mode: 'all',
		defaultValues: {
			displayName: '',
			email: '',
			id: '',
			username: '',
			description: '',
			password: '',
			image: null,
		},
		resolver: zodResolver(userSchema),
	})

	useEffect(() => {
		if (data) {
			form.reset(data, { keepDirty: false })
		}
	}, [data, form.reset])

	return (
		<div className="space-y-4">
			<FormServerError error={error} />
			<FormProvider {...form}>
				<div className="flex space-x-4 w-full">
					<FormField
						name="username"
						control={form.control}
						render={({
							field: { name, value, onChange, onBlur },
							fieldState: { isDirty, error, invalid: isError },
						}) => (
							<FormItem className="w-full">
								<FormControl>
									<>
										<FormLabelError
											htmlFor={name}
											label="Username"
											error={error?.message}
										/>
										<TextField
											type="text"
											value={value}
											onChange={(e) => onChange(e.target.value)}
											onBlur={onBlur}
											className={formClassnames({ isDirty, isError })}
											disabled={loading}
										/>
									</>
								</FormControl>
							</FormItem>
						)}
					/>
					<FormField
						name="displayName"
						control={form.control}
						render={({
							field: { name, value, onChange, onBlur },
							fieldState: { isDirty, error, invalid: isError },
						}) => (
							<FormItem className="w-full">
								<FormControl>
									<>
										<FormLabelError
											htmlFor={name}
											label="Display Name"
											error={error?.message}
										/>
										<TextField
											type="text"
											value={value}
											onChange={(e) => onChange(e.target.value)}
											onBlur={onBlur}
											className={formClassnames({ isDirty, isError })}
											disabled={loading}
										/>
									</>
								</FormControl>
							</FormItem>
						)}
					/>
				</div>
				<FormField
					name="password"
					control={form.control}
					render={({
						field: { name, value, onBlur, onChange },
						fieldState: { isDirty, error, invalid: isError },
					}) => (
						<FormItem className="w-full">
							<FormLabelError
								htmlFor={name}
								label="Password"
								error={error?.message}
							/>
							<FormControl>
								<TextField
									type="text"
									inputType="password"
									placeholder="Unchanged"
									value={value}
									onChange={(e) => onChange(e.target.value)}
									onBlur={onBlur}
									className={formClassnames({ isDirty, isError })}
									disabled={loading}
								/>
							</FormControl>
						</FormItem>
					)}
				/>
				<FormField
					name="email"
					control={form.control}
					render={({
						field: { name, value, onBlur, onChange },
						fieldState: { isDirty, error, invalid: isError },
					}) => (
						<FormItem className="w-full">
							<FormLabelError
								htmlFor={name}
								label="E-mail"
								error={error?.message}
							/>
							<FormControl>
								<TextField
									type="text"
									value={value}
									onChange={(e) => onChange(e.target.value)}
									onBlur={onBlur}
									className={formClassnames({ isDirty, isError })}
									disabled={loading}
								/>
							</FormControl>
						</FormItem>
					)}
				/>
				<FormField
					name="description"
					control={form.control}
					render={({
						field: { name, value, onChange, onBlur },
						fieldState: { isDirty, invalid: isError, error },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<>
									<FormLabelError
										htmlFor={name}
										label="Description"
										error={error?.message}
									/>
									<TextArea
										type="text"
										placeholder="Description"
										value={value}
										onChange={(e) => onChange(e.target.value)}
										onBlur={onBlur}
										className={formClassnames({ isDirty, isError })}
										disabled={loading}
									/>
								</>
							</FormControl>
						</FormItem>
					)}
				/>

				<FormImage form={form} required={false} />

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
					</div>
				</div>
			</FormProvider>
		</div>
	)
}
