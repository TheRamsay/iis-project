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
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	TextArea,
} from '@/components/components'
import { FormLabelError } from '@/app/_ui/form/form-label-error'

interface GroupSettingsFormProps {
	groupId: string
}

type Group = Pick<typeof schema.group.$inferSelect, 'id' | 'name'> & {
	visibility: 'public' | 'private'
	description: string
}

export type GroupForm = Pick<Group, 'id'> & Partial<Group>

export function GroupSettingsForm({ groupId }: GroupSettingsFormProps) {
	const { data, isFetching, refetch } = useQuery<Group>({
		queryKey: ['group-settings', groupId],
		queryFn: async () => {
			// TODO: endpoint
			await new Promise((resolve) => setTimeout(resolve, 1000))

			return {
				id: Math.random().toString(),
				name: 'groupdoe',
				visibility: 'public',
				description: 'BIO',
			}
		},
	})

	const { mutate, isPending } = useMutation({
		mutationKey: ['group-settings', groupId],
		mutationFn: async (data: GroupForm) => {
			// TODO: endpoint
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
			visibility: 'public',
			description: '',
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
				<FormField
					name="name"
					control={form.control}
					render={({
						field: { name, value, onChange, onBlur },
						fieldState: { isDirty, error },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<>
									<FormLabelError
										htmlFor={name}
										label="Description"
										error={error?.message}
									/>
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

				<FormField
					name="visibility"
					control={form.control}
					render={({
						field: { name, value, onChange },
						fieldState: { isDirty },
						formState: { disabled },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<div className="flex flex-row items-center space-x-4">
									<label htmlFor={name}>Visibility</label>
									<Select
										value={value}
										onValueChange={onChange}
										disabled={disabled}
									>
										<SelectTrigger
											className={formClassnames(
												{ isDirty },
												'flex justify-between w-full',
											)}
										>
											{value === 'private' ? 'Private' : 'Public'}
										</SelectTrigger>
										<SelectContent>
											<SelectItem value="private">Private</SelectItem>
											<SelectItem value="public">Public</SelectItem>
										</SelectContent>
									</Select>
								</div>
							</FormControl>
						</FormItem>
					)}
				/>

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
