'use client'

import { backendFetch } from '@/app/_lib/backend-fetch'
import { BACKEND_URL } from '@/app/_lib/constants'
import { formClassnames } from '@/app/_lib/form-classnames'
import { myz } from '@/app/_types/zod'
import { FormLabelError } from '@/app/_ui/form/form-label-error'
import { FormServerError } from '@/app/_ui/form/form-server-error'
import {
	Button,
	FormControl,
	FormField,
	FormItem,
	Loader,
	TextField,
} from '@/components/components'
import { zodResolver } from '@hookform/resolvers/zod'
import { useMutation } from '@tanstack/react-query'
import classNames from 'classnames'
import { useRouter } from 'next/navigation'
import { FormProvider, useForm } from 'react-hook-form'
import { z, type ZodType } from 'zod'

const registerSchema: ZodType<FormRegister> = z.object({
	email: z.string().email(),
	username: myz.username,
	password: myz.password,
})

type FormRegister = {
	email: string
	username: string
	password: string
}

export function FormRegister() {
	const { push } = useRouter()

	const form = useForm<FormRegister>({
		mode: 'all',
		defaultValues: {
			email: '',
			password: '',
			username: '',
		},
		resolver: zodResolver(registerSchema),
	})

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['register'],
		mutationFn: async (formData: FormRegister) => {
			const response = await backendFetch('/api/auth/register', {
				method: 'POST',
				body: JSON.stringify(formData),
			})

			if (!response.ok) {
				const data = await response.json()

				if (data.error) {
					throw new Error(data.error)
				}

				throw new Error('An unknown error has occurred.')
			}
		},
		onSuccess: () => {
			push('/settings')
		},
		onError: (error) => {
			// TODO: Error handling
			form.setError('username', { message: 'This username already exists.' })
			form.setError('email', { message: 'This email already exists.' })
		},
	})

	const loading = isPending

	return (
		<div className="space-y-4">
			<FormServerError error={error} />
			<FormProvider {...form}>
				<FormField
					name="username"
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
										label="Username"
										error={error?.message}
									/>
									<TextField
										type="text"
										placeholder="honza_strelec"
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
			</FormProvider>
			<FormProvider {...form}>
				<FormField
					name="email"
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
										label="E-mail"
										error={error?.message}
									/>
									<TextField
										type="text"
										placeholder="user@example.com"
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
			</FormProvider>
			<FormProvider {...form}>
				<FormField
					name="password"
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
										label="Password"
										error={error?.message}
									/>
									<TextField
										inputType="password"
										placeholder="hunter2"
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

				<div className="flex flex-row w-full justify-between items-center">
					<div className={classNames(!loading && 'hidden')}>
						<Loader size={20} />
					</div>
					<div className="flex w-full justify-end space-x-4">
						<Button
							onClick={() => mutate(form.watch())}
							disabled={loading || !form.formState.isDirty}
						>
							Register
						</Button>
					</div>
				</div>
			</FormProvider>
		</div>
	)
}
