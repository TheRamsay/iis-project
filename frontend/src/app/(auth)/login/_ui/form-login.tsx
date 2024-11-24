'use client'

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
import Link from 'next/link'
import { useRouter } from 'next/navigation'
import { FormProvider, useForm } from 'react-hook-form'
import { z, type ZodType } from 'zod'

const loginSchema: ZodType<FormLogin> = z.object({
	username: myz.username,
	password: myz.password,
})

type FormLogin = {
	username: string
	password: string
}

export function FormLogin() {
	const { refresh } = useRouter()

	const form = useForm<FormLogin>({
		mode: 'all',
		defaultValues: {
			username: '',
			password: '',
		},
		resolver: zodResolver(loginSchema),
	})

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['login'],
		mutationFn: async (formData: FormLogin) => {
			const response = await fetch(`${BACKEND_URL}/api/auth/login`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(formData),
				credentials: 'include',
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
			refresh()
		},
		onError: (error) => {
			// TODO: Error handling
			form.setError('username', { message: 'Invalid username.' })
			form.setError('password', { message: 'Invalid password.' })
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
										label="E-mail"
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

				<div className="flex flex-row w-full justify-between items-center space-x-3">
					<Link href="/register">
						<Button variant="outline">Register</Button>
					</Link>
					<div className={classNames(!loading && 'hidden')}>
						<Loader size={20} />
					</div>
					<div className="flex w-full justify-end space-x-4">
						<Button
							onClick={() => mutate(form.watch())}
							disabled={loading || !form.formState.isDirty}
						>
							Login
						</Button>
					</div>
				</div>
			</FormProvider>
		</div>
	)
}
