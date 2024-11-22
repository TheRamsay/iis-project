'use client'

import { formClassnames } from '@/app/_lib/form-classnames'
import { TextField } from '@/components/components/text-field'
import { FormControl, FormField, FormItem } from '@/components/components/form'
import type { UseFormReturn } from 'react-hook-form'
import { z } from 'zod'
import { useEffect } from 'react'
import { FormLabelError } from '@/app/_ui/form/form-label-error'

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

// https://stackoverflow.com/a/31408260/14021198
const latRegex = RegExp(
	/^(\+|-)?(?:90(?:(?:\.0{1,6})?)|(?:[0-9]|[1-8][0-9])(?:(?:\.[0-9]{1,6})?))$/,
)
const lngRegex = RegExp(
	/^(\+|-)?(?:180(?:(?:\.0{1,6})?)|(?:[0-9]|[1-9][0-9]|1[0-7][0-9])(?:(?:\.[0-9]{1,6})?))$/,
)

export const formLocationSchema = z.object({
	location: z
		.object({
			lat: z.string(),
			lng: z.string(),
		})
		.superRefine((data, ctx) => {
			if (data.lat && !latRegex.test(data.lat)) {
				ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: 'Invalid LAT',
					path: ['lat'],
				})
			}

			if (data.lng && !lngRegex.test(data.lng)) {
				ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: 'Invalid LNG',
					path: ['lng'],
				})
			}

			if (data.lat !== '' && data.lng === '') {
				return ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: 'LNG is required',
					path: ['lng'],
					fatal: true,
				})
			}

			if (data.lat === '' && data.lng !== '') {
				return ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: 'LAT is required',
					path: ['lat'],
					fatal: true,
				})
			}
		}),
})

export function FormLocation<T extends FormSubset>({
	form: _form,
	disabled = false,
}: FormLocation<T>) {
	const form = _form as unknown as UseFormReturn<FormSubset>

	const { lat, lng } = form.watch('location')

	useEffect(() => {
		form.trigger('location.lng')
		form.trigger('location.lat')

		if (lat && lng) {
		}
	}, [lat, lng, form.trigger])

	return (
		<div className="space-y-1">
			<span>Location</span>
			<div className="space-x-4 flex">
				<FormField
					name="location.lat"
					control={form.control}
					render={({
						field: { name, value, onBlur, onChange },
						fieldState: { isDirty, invalid: isError, error },
					}) => (
						<FormItem className="w-full">
							<FormLabelError
								htmlFor={name}
								label="LAT"
								error={error?.message}
								className="text-sm"
							/>
							<FormControl>
								<TextField
									type="number"
									placeholder="49.19481"
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
						fieldState: { isDirty, invalid: isError, error },
					}) => (
						<FormItem className="w-full">
							<FormLabelError
								htmlFor={name}
								label="LNG"
								error={error?.message}
								className="text-sm"
							/>
							<FormControl>
								<TextField
									type="number"
									placeholder="16.60859"
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
		</div>
	)
}
