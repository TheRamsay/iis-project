'use client'

import { FormControl, FormField, FormItem } from '@/components/components/form'
import type { UseFormReturn } from 'react-hook-form'
import type { UserForm } from './user-modal'
import { TextField } from '@/components/components/text-field'
import { Checkbox } from '@/components/components/checkbox'
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '@/components/components/select'
import { userType } from '../../../../../../drizzle/schema'
import { formClassnames } from '@/app/_lib/form-classnames'
import { FormImage } from '@/app/_ui/form/form-image'
import { FormLabelError } from '@/app/_ui/form/form-label-error'
import { TextArea } from '@/components/components'
import { useSession } from '@/app/_lib/auth/auth-provider'
import classNames from 'classnames'

export function UserModalForm({ form }: { form: UseFormReturn<UserForm> }) {
	const session = useSession()
	const isModerator = session?.role === 'moderator'

	return (
		<>
			{isModerator && (
				<div className="text-blue-500 bg-blue-500 bg-opacity-30 border border-blue-500 px-4 py-2 rounded-xl">
					You are a moderator, so you can only un/block the user.
				</div>
			)}
			<FormField
				control={form.control}
				name="id"
				render={({ field: { name, value } }) => (
					<FormItem>
						<FormControl>
							<>
								<label htmlFor={name}>ID</label>
								<TextField
									type="text"
									value={value}
									disabled
									className="!text-opacity-50"
								/>
							</>
						</FormControl>
					</FormItem>
				)}
			/>
			<div className="flex space-x-4 w-full">
				<FormField
					control={form.control}
					name="username"
					render={({
						field: { name, value, onChange, onBlur },
						fieldState: { isDirty },
						formState: { disabled },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<>
									<label htmlFor={name}>Username</label>
									<TextField
										type="text"
										value={value || ''}
										onChange={onChange}
										onBlur={onBlur}
										className={formClassnames({ isDirty })}
										disabled={disabled || isModerator}
									/>
								</>
							</FormControl>
						</FormItem>
					)}
				/>
			</div>
			<FormField
				name="description"
				control={form.control}
				render={({
					field: { name, value, onChange, onBlur },
					fieldState: { isDirty, invalid: isError, error },
					formState: { disabled },
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
									disabled={disabled || isModerator}
								/>
							</>
						</FormControl>
					</FormItem>
				)}
			/>
			<FormField
				control={form.control}
				name="email"
				render={({
					field: { name, value, onBlur, onChange },
					fieldState: { isDirty, error, invalid: isError },
					formState: { disabled },
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
									value={value || ''}
									onChange={onChange}
									onBlur={onBlur}
									className={formClassnames({ isDirty, isError })}
									disabled={disabled || isModerator}
								/>
							</>
						</FormControl>
					</FormItem>
				)}
			/>
			<FormImage form={form} required={false} disabled={isModerator} />
			<div className="flex flex-row justify-between items-center">
				<FormField
					control={form.control}
					name="isBlocked"
					render={({
						field: { name, value, onBlur, onChange },
						fieldState: { isDirty },
						formState: { disabled },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<div className="flex flex-row items-center space-x-4">
									<label htmlFor={name}>Blocked</label>
									<Checkbox
										checked={value}
										onBlur={onBlur}
										onCheckedChange={onChange}
										className={formClassnames(
											{ isDirty },
											!isDirty && '!border-accent',
											' text-black text-opacity-50',
										)}
										disabled={disabled}
									/>
								</div>
							</FormControl>
						</FormItem>
					)}
				/>
				<FormField
					control={form.control}
					name="role"
					render={({
						field: { name, value, onChange },
						fieldState: { isDirty },
						formState: { disabled },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<div
									className={classNames('flex flex-row items-center space-x-4')}
								>
									<label htmlFor={name}>Role</label>
									<Select
										value={value}
										onValueChange={onChange}
										disabled={disabled || isModerator}
									>
										<SelectTrigger
											className={formClassnames(
												{ isDirty },
												'flex justify-between w-full',
											)}
										>
											<SelectValue />
										</SelectTrigger>
										<SelectContent>
											{userType.enumValues.map((role) => (
												<SelectItem key={role} value={role}>
													{role}
												</SelectItem>
											))}
										</SelectContent>
									</Select>
								</div>
							</FormControl>
						</FormItem>
					)}
				/>
			</div>
		</>
	)
}
