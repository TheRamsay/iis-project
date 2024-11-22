'use client'

import type { schema } from '@/app/_lib/db'
import { formClassnames } from '@/app/_lib/form-classnames'
import { TextField } from '@/components/components/text-field'
import { FormControl, FormField, FormItem } from '@/components/components/form'
import { FormProvider, useForm } from 'react-hook-form'
import { Button } from '@/components/components/button'
import { useMutation } from '@tanstack/react-query'
import { Loader } from '@/components/components/loader'
import classNames from 'classnames'
import { TextArea } from '@/components/components/text-area'
import { FormImage, formImageSchema } from '../../../_ui/form/form-image'
import type { Entity } from '../../_ui/pick-entities'
import { FormVisibility, formVisibilitySchema } from '../../_ui/form-visibility'
import { FormLocation, formLocationSchema } from '../../_ui/form-location'
import { z, type ZodType } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { FormTags, formTagsSchema } from '../../_ui/form-tags'
import { FormLabelError } from '@/app/_ui/form/form-label-error'

const submitPostFromSchema: ZodType<PostForm> = z
	.object({
		title: z.string().min(3).max(64),
		description: z.string().max(255),
	})
	.merge(formImageSchema(true))
	.merge(formLocationSchema)
	.merge(formVisibilitySchema)
	.merge(formTagsSchema)

type Post = Pick<typeof schema.post.$inferSelect, 'description' | 'title'> & {
	visibility: 'public' | 'private'
	image: globalThis.File | null
	location: {
		lat: string
		lng: string
	}
	allowedUsers: Entity[]
	allowedGroups: Entity[]
	tags: string[]
}

export type PostForm = Post

export function SubmitPostForm() {
	const { mutate, isPending } = useMutation({
		mutationKey: ['submit-post'],
		mutationFn: async (data: PostForm) => {
			console.log('mutation')
			await new Promise((resolve) => setTimeout(resolve, 1000))
		},
		onSuccess: () => {
			// TODO: goto profile?
		},
	})

	const loading = isPending

	const form = useForm<PostForm>({
		mode: 'all',
		defaultValues: {
			description: '',
			location: { lat: '', lng: '' },
			title: '',
			visibility: 'public',
			tags: [],
			image: null,
			allowedGroups: [],
			allowedUsers: [
				{
					avatar: {
						src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
						width: 128,
						height: 128,
					},
					id: '1',
					username: 'John Doe',
				},
			],
		},
		resolver: zodResolver(submitPostFromSchema),
	})

	return (
		<div className="space-y-4">
			<FormProvider {...form}>
				<FormField
					name="title"
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
										label="Title*"
										error={error?.message}
									/>
									<TextField
										type="text"
										placeholder="Title"
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
				<FormField
					name="description"
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
										disabled={loading}
									/>
								</>
							</FormControl>
						</FormItem>
					)}
				/>

				<FormLocation form={form} />
				<FormVisibility form={form} />

				<FormTags form={form} />

				<FormImage form={form} />

				<div className="flex flex-row w-full justify-between items-center">
					<div className={classNames(!loading && 'hidden')}>
						<Loader size={20} />
					</div>
					<div className="flex w-full justify-end space-x-4">
						<Button
							onClick={form.handleSubmit((data) => {
								mutate(data)
							})}
							disabled={
								loading || Object.values(form.formState.errors).some(Boolean)
							}
						>
							Submit
						</Button>
					</div>
				</div>
			</FormProvider>
		</div>
	)
}
