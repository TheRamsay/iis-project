'use client'

import { FormControl, FormField, FormItem } from '@/components/components/form'
import { ChipInput } from '@/components/components/chip-input'
import type { UseFormReturn } from 'react-hook-form'
import { z } from 'zod'

export const formTagsSchema = z.object({
	tags: z.array(z.string()),
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
				fieldState: { invalid: isError },
			}) => (
				<FormItem className="w-full">
					<label htmlFor={name}>Tags</label>
					<FormControl>
						<ChipInput
							values={value}
							onValueChange={onChange}
							placeholder="Tags"
						/>
					</FormControl>
				</FormItem>
			)}
		/>
	)
}
