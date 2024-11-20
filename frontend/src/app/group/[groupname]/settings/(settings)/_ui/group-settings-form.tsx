'use client'

import type { schema } from '@/app/_lib/db'
import { formClassnames } from '@/app/_lib/form-classnames'
import { TextField } from '@/components/components/text-field'
import { FormControl, FormField, FormItem } from '@/components/components/form'
import { FormProvider, useForm } from 'react-hook-form'
import { Button } from '@/components/components/button'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useEffect } from 'react'
import { Loader } from '@/components/components/loader'
import classNames from 'classnames'

interface GroupSettingsFormProps {
	groupId: string
}

type Group = Pick<typeof schema.group.$inferSelect, 'id' | 'name'>

export type GroupForm = Pick<Group, 'id'> & Partial<Group>

export function GroupSettingsForm({ groupId }: GroupSettingsFormProps) {
	const { data, isFetching, refetch } = useQuery<Group>({
		queryKey: ['group-settings', groupId],
		queryFn: async () => {
			await new Promise((resolve) => setTimeout(resolve, 1000))

			return {
				id: Math.random().toString(),
				name: 'groupdoe',
			}
		},
	})

	const { mutate, isPending } = useMutation({
		mutationKey: ['group-settings', groupId],
		mutationFn: async (data: GroupForm) => {
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onSuccess: () => {
			refetch()
		},
	})

	const loading = isFetching || isPending

	const form = useForm<GroupForm>({
		defaultValues: {
			id: '',
			name: '',
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
						name="name"
						control={form.control}
						render={({
							field: { name, value, onChange, onBlur },
							formState: { isDirty },
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
					{/* <FormField
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
					/> */}
				</div>
				{/* <FormField
					name="avatarUrl"
					control={form.control}
					render={({
						field: { name, value, onBlur, onChange },
						fieldState: { isDirty },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<>
									<label htmlFor={name}>Avatar</label>
									<TextField
										type="text"
										value={value || ''}
										onChange={(e) => onChange(e.target.value)}
										onBlur={onBlur}
										className={formClassnames({ isDirty })}
										disabled={loading}
									/>
								</>
							</FormControl>
						</FormItem>
					)}
				/> */}
				<div className="flex flex-row w-full justify-between items-center">
					<div className={classNames(!loading && 'hidden')}>
						<Loader size={20} />
					</div>
					<div className="flex w-full justify-end space-x-4">
						<Button
						// onClick={() => mutate(form.watch())}
						// disabled={loading || !form.formState.isDirty}
						>
							Save
						</Button>
					</div>
				</div>
			</FormProvider>
		</div>
	)
}
