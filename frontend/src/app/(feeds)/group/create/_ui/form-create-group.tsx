'use client'

import { formClassnames } from '@/app/_lib/form-classnames'
import { TextField } from '@/components/components/text-field'
import { FormControl, FormField, FormItem } from '@/components/components/form'
import { FormProvider, useForm } from 'react-hook-form'
import { Button } from '@/components/components/button'
import { useMutation } from '@tanstack/react-query'
import { Loader } from '@/components/components/loader'
import classNames from 'classnames'
import { z, type ZodType } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { FormLabelError } from '@/app/_ui/form/form-label-error'
import { FormServerError } from '@/app/_ui/form/form-server-error'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'
import { useRouter } from 'next/navigation'
import { myz } from '@/app/_types/zod'

const createGroupFormSchema: ZodType<CreateGroupForm> = z.object({
	name: myz.username,
})

type CreateGroupForm = {
	name: string
}

export function FormCreateGroup() {
	const { push } = useRouter()

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['create-group'],
		mutationFn: async (formData: CreateGroupForm) => {
			const response = await backendFetch('/api/groups', {
				method: 'POST',
				body: JSON.stringify({
					name: formData.name,
				}),
			})

			try {
				await checkResponse(response, { passError: true })
			} catch (error) {
				let message = 'An unknown error has occured.'

				try {
					if (error instanceof Error) {
						const data = JSON.parse(error.message)
						message = data.name[0].message
					}
				} catch {}

				throw new Error(message)
			}

			return {
				name: formData.name,
			}
		},
		onSuccess: (res) => {
			push(`/group/${res.name}`)
		},
	})

	const loading = isPending

	const form = useForm<CreateGroupForm>({
		mode: 'all',
		defaultValues: {
			name: '',
		},
		resolver: zodResolver(createGroupFormSchema),
	})

	return (
		<div className="space-y-4">
			<FormServerError error={error} />
			<FormProvider {...form}>
				<FormField
					name="name"
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
										label="Group Name"
										error={error?.message}
									/>
									<TextField
										type="text"
										placeholder="Title"
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

				<div className="flex flex-row w-full justify-between items-center">
					<div className="flex w-full justify-end space-x-4 items-center">
						<div className={classNames(!loading && 'hidden')}>
							<Loader size={20} />
						</div>
						<Button
							onClick={form.handleSubmit((data) => {
								mutate(data)
							})}
							disabled={
								loading || Object.values(form.formState.errors).some(Boolean)
							}
						>
							Create
						</Button>
					</div>
				</div>
			</FormProvider>
		</div>
	)
}
