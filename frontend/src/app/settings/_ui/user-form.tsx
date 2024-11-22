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
import { FormImage } from '@/app/_ui/form/form-image'
import { FormLabelError } from '@/app/_ui/form/form-label-error'
import { TextArea } from '@/components/components'

interface UserFormProps {
	userId: string
}

type User = Pick<
	typeof schema.user.$inferSelect,
	'id' | 'displayName' | 'avatarUrl' | 'email' | 'username'
> & { image: globalThis.File | null; description: string }

export type UserForm = Pick<User, 'id'> & Partial<User>

export function UserForm({ userId }: UserFormProps) {
	const { data, isFetching, refetch } = useQuery<User>({
		queryKey: ['admin-user', userId],
		queryFn: async () => {
			await new Promise((resolve) => setTimeout(resolve, 1000))

			return {
				id: Math.random().toString(),
				displayName: 'John Doe',
				avatarUrl: 'https://example.com/favicon.ico',
				email: 'asdas@goog.eoco',
				username: 'johndoe',
				description: 'BIO',
				image: null,
			}
		},
	})

	const { mutate, isPending } = useMutation({
		mutationKey: ['admin-user', userId],
		mutationFn: async (data: UserForm) => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onSuccess: () => {
			refetch()
		},
	})

	const loading = isFetching || isPending

	const form = useForm<UserForm>({
		defaultValues: {
			avatarUrl: '',
			displayName: '',
			email: '',
			id: '',
			username: '',
			description: '',
			image: null,
		},
	})

	useEffect(() => {
		if (data) {
			form.reset(data, { keepDirty: false })
		}
	}, [data, form.reset])

	return (
		<div className="space-y-4">
			<FormProvider {...form}>
				<div className="flex space-x-4 w-full">
					<FormField
						name="username"
						control={form.control}
						render={({
							field: { name, value, onChange, onBlur },
							fieldState: { isDirty },
						}) => (
							<FormItem className="w-full">
								<FormControl>
									<>
										<label htmlFor={name}>Username</label>
										<TextField
											type="text"
											value={value}
											onChange={(e) => onChange(e.target.value)}
											onBlur={onBlur}
											className={formClassnames({ isDirty })}
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
							fieldState: { isDirty },
						}) => (
							<FormItem className="w-full">
								<FormControl>
									<>
										<label htmlFor={name}>Display Name</label>
										<TextField
											type="text"
											value={value}
											onChange={(e) => onChange(e.target.value)}
											onBlur={onBlur}
											className={formClassnames({ isDirty })}
											disabled={loading}
										/>
									</>
								</FormControl>
							</FormItem>
						)}
					/>
				</div>
				<FormField
					name="email"
					control={form.control}
					render={({
						field: { name, value, onBlur, onChange },
						fieldState: { isDirty },
					}) => (
						<FormItem className="w-full">
							<label htmlFor={name}>E-mail</label>
							<FormControl>
								<TextField
									type="text"
									value={value}
									onChange={(e) => onChange(e.target.value)}
									onBlur={onBlur}
									className={formClassnames({ isDirty })}
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
