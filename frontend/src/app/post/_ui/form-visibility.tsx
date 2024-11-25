'use client'

import { formClassnames } from '@/app/_lib/form-classnames'
import { FormControl, FormField, FormItem } from '@/components/components/form'
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from '@/components/components/select'
import type { UseFormReturn } from 'react-hook-form'
import { type Entity, PickEntities } from './pick-entities'
import { Collapsible } from '@/components/components/animation/Collapsible'
import { z } from 'zod'

const entitySchema = z.object({
	id: z.string(),
	username: z.string(),
	avatar: z.object({
		// biome-ignore lint/suspicious/noExplicitAny: <explanation>
		src: z.string().optional().nullable() as any,
	}),
})

export const formVisibilitySchema = z.object({
	visibility: z.enum(['public', 'private']),
	allowedUsers: z.array(entitySchema),
	allowedGroups: z.array(entitySchema),
})

type FormSubset = {
	visibility: 'public' | 'private'
	allowedUsers: Entity[]
	allowedGroups: Entity[]
}

interface FormVisibility<T extends FormSubset> {
	form: UseFormReturn<T>
}

export function FormVisibility<T extends FormSubset>({
	form: _form,
}: FormVisibility<T>) {
	const form = _form as unknown as UseFormReturn<FormSubset>

	return (
		<>
			<FormField
				name="visibility"
				control={form.control}
				render={({
					field: { name, value, onChange },
					fieldState: { isDirty },
					formState: { disabled },
				}) => (
					<FormItem className="w-full">
						<FormControl>
							<div className="flex flex-row items-center space-x-4">
								<label htmlFor={name}>Visibility</label>
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
										{value === 'private' ? 'Private' : 'Public'}
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="private">Private</SelectItem>
										<SelectItem value="public">Public</SelectItem>
									</SelectContent>
								</Select>
							</div>
						</FormControl>
					</FormItem>
				)}
			/>
			<span>
				<Collapsible
					open={form.watch('visibility') === 'private'}
					className="grid grid-cols-2 gap-4 pt-2"
				>
					<PickEntities
						type="user"
						list={form.watch('allowedUsers')}
						onChange={(newList) =>
							form.setValue('allowedUsers', newList, {
								shouldTouch: true,
								shouldValidate: true,
							})
						}
					/>
					<PickEntities
						type="group"
						list={form.watch('allowedGroups')}
						onChange={(newList) =>
							form.setValue('allowedGroups', newList, {
								shouldTouch: true,
								shouldValidate: true,
							})
						}
					/>
				</Collapsible>
				<Collapsible
					open={form.watch('visibility') === 'public'}
					className="pt-2"
				>
					<PickEntities
						type="group"
						list={form.watch('allowedGroups')}
						onChange={(newList) =>
							form.setValue('allowedGroups', newList, {
								shouldTouch: true,
								shouldValidate: true,
							})
						}
					/>
				</Collapsible>
			</span>
		</>
	)
}
