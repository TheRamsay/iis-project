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
import { FormServerError } from '@/app/_ui/form/form-server-error'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'
import { useRouter } from 'next/navigation'
import { uploadImage } from '@/app/_lib/upload-image'
import { myz } from '@/app/_types/zod'

const submitPostFromSchema: ZodType<PostForm> = z
	.object({
		title: myz.title,
		description: myz.description,
	})
	.merge(formImageSchema(true))
	// .merge(formLocationSchema)
	.merge(formVisibilitySchema)
	.merge(formTagsSchema)

type Post = Pick<typeof schema.post.$inferSelect, 'description' | 'title'> & {
	visibility: 'public' | 'private'
	image: string | null
	// location: {
	// 	lat: string
	// 	lng: string
	// }
	allowedUsers: Entity[]
	allowedGroups: Entity[]
	tags: string[]
}

export type PostForm = Post

export function SubmitPostForm() {
	const { push } = useRouter()

	const { mutate, error, isPending } = useMutation({
		mutationKey: ['submit-post'],
		mutationFn: async (formData: PostForm) => {
			if (!formData.image) {
				throw new Error('Image is required')
			}
			const { link } = await uploadImage(formData.image)

			const response = await backendFetch('/api/posts', {
				method: 'POST',
				body: JSON.stringify({
					title: formData.title,
					description: formData.description,
					post_type: 'photo',
					visibility: formData.visibility,
					content_url: link,
					tags: formData.tags,
					allowed_users:
						formData.visibility === 'private'
							? formData.allowedUsers.map((u) => u.id)
							: undefined,
					allowed_groups: formData.allowedGroups.map((g) => g.id),
				}),
			})

			await checkResponse(response)
			return response.json() as Promise<{ id: string }>
		},
		onSuccess: (res) => {
			push(`/post/${res.id}`)
		},
		onError: () => {
			scroll?.({ top: 0, behavior: 'smooth' })
		},
	})

	const loading = isPending

	const form = useForm<PostForm>({
		mode: 'all',
		defaultValues: {
			description: '',
			// location: { lat: '', lng: '' },
			title: '',
			visibility: 'public',
			tags: [],
			image: null,
			allowedGroups: [],
			allowedUsers: [],
		},
		resolver: zodResolver(submitPostFromSchema),
	})

	console.log(form.formState.errors)

	return (
		<div className="space-y-4">
			<FormServerError error={error} />
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
										label="Title"
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

				{/* <FormLocation form={form} /> */}
				<FormVisibility form={form} />

				<FormTags form={form} />

				<FormImage form={form} />

				<div className="flex flex-row w-full justify-between items-center">
					<div className="flex w-full justify-end space-x-4 items-center">
						<div className={classNames(!loading && 'hidden')}>
							<Loader size={20} />
						</div>
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
