'use client'

import { formClassnames } from '@/app/_lib/form-classnames'
import { TextField } from '@/components/components/text-field'
import { FormControl, FormField, FormItem } from '@/components/components/form'
import type { UseFormReturn } from 'react-hook-form'
import { z } from 'zod'

type FormSubset = {
	location: {
		lat: string
		lng: string
	}
}

interface FormLocation<T extends FormSubset> {
	form: UseFormReturn<T>
	disabled?: boolean
}

export const formLocationSchema = z.object({
	location: z.object({
		lat: z.string(),
		lng: z.string(),
	}),
})

export function FormLocation<T extends FormSubset>({
	form: _form,
	disabled = false,
}: FormLocation<T>) {
	const form = _form as unknown as UseFormReturn<FormSubset>

	return (
		<div className="space-x-4 flex">
			<FormField
				name="location.lat"
				control={form.control}
				render={({
					field: { name, value, onBlur, onChange },
					fieldState: { isDirty, invalid: isError },
				}) => (
					<FormItem className="w-full">
						<label htmlFor={name}>Location (LAT)</label>
						<FormControl>
							<TextField
								type="number"
								placeholder="49.194808509"
								value={value}
								onValueChange={(value) => onChange(value)}
								onBlur={onBlur}
								className={formClassnames({ isDirty, isError })}
								disabled={disabled}
							/>
						</FormControl>
					</FormItem>
				)}
			/>
			<FormField
				name="location.lng"
				control={form.control}
				render={({
					field: { name, value, onBlur, onChange },
					fieldState: { isDirty, invalid: isError },
				}) => (
					<FormItem className="w-full">
						<label htmlFor={name}>Location (LNG)</label>
						<FormControl>
							<TextField
								type="number"
								placeholder="16.608591921"
								value={value}
								onValueChange={(value) => onChange(value)}
								onBlur={onBlur}
								className={formClassnames({ isDirty, isError })}
								disabled={disabled}
							/>
						</FormControl>
					</FormItem>
				)}
			/>
		</div>
	)
}
