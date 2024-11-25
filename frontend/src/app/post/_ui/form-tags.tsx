'use client'

import { FormControl, FormField, FormItem } from '@/components/components/form'
import { ChipInput } from '@/components/components/chip-input'
import type { UseFormReturn } from 'react-hook-form'
import { z } from 'zod'
import { formClassnames } from '@/app/_lib/form-classnames'
import { FormLabelError } from '@/app/_ui/form/form-label-error'

export const formTagsSchema = z.object({
	tags: z.array(z.string()).refine(
		(tags) => {
			const set = new Set(tags)
			console.log(set.size, tags.length)
			return set.size === tags.length
		},
		{
			message: 'Tags must be unique',
		},
	),
})

type FormSubset = {
	tags: string[]
}

interface FormLocation<T extends FormSubset> {
	form: UseFormReturn<T>
	disabled?: boolean
}

export function FormTags<T extends FormSubset>({
	form: _form,
}: FormLocation<T>) {
	const form = _form as unknown as UseFormReturn<FormSubset>

	return (
		<FormField
			name="tags"
			control={form.control}
			render={({
				field: { name, value, onChange },
				fieldState: { isDirty, invalid: isError, error },
			}) => (
				<FormItem className="w-full">
					<FormLabelError htmlFor={name} label="Tags" error={error?.message} />
					<FormControl>
						<div className={formClassnames({ isDirty, isError }, 'rounded-xl')}>
							<ChipInput
								values={value}
								onValueChange={onChange}
								placeholder="Tags"
							/>
						</div>
					</FormControl>
				</FormItem>
			)}
		/>
	)
}
