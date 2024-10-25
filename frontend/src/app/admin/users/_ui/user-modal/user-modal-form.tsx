'use client'

import { FormControl, FormField, FormItem } from '@/components/components/form'
import { useFormContext } from 'react-hook-form'
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

export function UserModalForm() {
	const { control } = useFormContext<UserForm>()

	return (
		<>
			<FormField
				control={control}
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
					control={control}
					name="username"
					render={({
						field: { name, value, onChange, onBlur },
						formState: { isDirty },
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
										disabled={disabled}
									/>
								</>
							</FormControl>
						</FormItem>
					)}
				/>
				<FormField
					control={control}
					name="displayName"
					render={({
						field: { name, value, onChange, onBlur },
						fieldState: { isDirty },
						formState: { disabled },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<>
									<label htmlFor={name}>Display Name</label>
									<TextField
										type="text"
										value={value || ''}
										onChange={onChange}
										onBlur={onBlur}
										className={formClassnames({ isDirty })}
										disabled={disabled}
									/>
								</>
							</FormControl>
						</FormItem>
					)}
				/>
			</div>
			<FormField
				control={control}
				name="email"
				render={({
					field: { name, value, onBlur, onChange },
					fieldState: { isDirty },
					formState: { disabled },
				}) => (
					<FormItem className="w-full">
						<FormControl>
							<>
								<label htmlFor={name}>E-mail</label>
								<TextField
									type="text"
									value={value || ''}
									onChange={onChange}
									onBlur={onBlur}
									className={formClassnames({ isDirty })}
									disabled={disabled}
								/>
							</>
						</FormControl>
					</FormItem>
				)}
			/>
			<FormField
				control={control}
				name="avatarUrl"
				render={({
					field: { name, value, onBlur, onChange },
					fieldState: { isDirty },
					formState: { disabled },
				}) => (
					<FormItem className="w-full">
						<FormControl>
							<>
								<label htmlFor={name}>Avatar</label>
								<TextField
									type="text"
									value={value || ''}
									onChange={onChange}
									onBlur={onBlur}
									className={formClassnames({ isDirty })}
									disabled={disabled}
								/>
							</>
						</FormControl>
					</FormItem>
				)}
			/>
			<div className="flex flex-row justify-between items-center">
				<FormField
					control={control}
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
					control={control}
					name="userType"
					render={({
						field: { name, value, onChange },
						fieldState: { isDirty },
						formState: { disabled },
					}) => (
						<FormItem className="w-full">
							<FormControl>
								<div className="flex flex-row items-center space-x-4">
									<label htmlFor={name}>Role</label>
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
