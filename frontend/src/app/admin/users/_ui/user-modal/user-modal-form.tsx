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

export function UserModalForm({ form }: { form: UseFormReturn<UserForm> }) {
	return (
		<>
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
										disabled={disabled}
									/>
								</>
							</FormControl>
						</FormItem>
					)}
				/>
				{/* <FormField
					control={form.control}
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
				/> */}
			</div>
			<FormField
				control={form.control}
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
			<FormImage form={form} required={false} />
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
