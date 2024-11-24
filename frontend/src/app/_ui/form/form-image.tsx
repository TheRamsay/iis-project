'use client'

import { FormControl, FormField, FormItem } from '@/components/components'
import classNames from 'classnames'
import { useCallback, useEffect, useState } from 'react'
import { useDropzone } from 'react-dropzone'
import type { UseFormReturn } from 'react-hook-form'
import { z } from 'zod'
import { FormLabelError } from './form-label-error'
import { formClassnames } from '@/app/_lib/form-classnames'
import { XIcon } from 'lucide-react'

export const formImageSchema = (required: boolean) =>
	z.object({
		image: z
			.custom<globalThis.File>(
				(data) => {
					console.log(!!data && required)
					return !!data && required
				},
				{ fatal: true, message: 'Image is required' },
			)
			.refine(
				(data) => {
					if (!data && !required) return true
					return data.type.startsWith('image/')
				},
				{ message: 'File must be an image' },
			),
	})

interface FormSubset {
	image?: globalThis.File | null | undefined
}

interface FormImage<T extends FormSubset> {
	form: UseFormReturn<T>
	required?: boolean
	className?: string
}

export function FormImage<T extends FormSubset>({
	form: _form,
	required = true,
	className,
}: FormImage<T>) {
	const form = _form as unknown as UseFormReturn<FormSubset>
	const [initialPreview, setInitialPreview] = useState<string | undefined>()
	const [preview, setPreview] = useState<string | undefined>()

	const deleteImage = useCallback(() => {
		// TODO: Test dirtying
		form.setValue('image', null, {
			shouldDirty: !form.control._defaultValues.image,
		})
		setPreview(undefined)
	}, [form.setValue, form.control._defaultValues.image])

	const { getRootProps, getInputProps } = useDropzone({
		accept: {
			'image/*': [],
		},
		maxFiles: 1,
		multiple: false,
		onDrop: ([file]) => {
			if (!file) return

			const newObjectURL = URL.createObjectURL(file)

			if (initialPreview === newObjectURL) {
				form.setValue('image', file, {
					shouldDirty: false,
					shouldValidate: true,
				})
			} else {
				form.setValue('image', file, {
					shouldDirty: true,
					shouldValidate: true,
				})
			}

			setPreview(() => {
				if (preview) {
					URL.revokeObjectURL(preview)
				}
				return newObjectURL
			})
		},
	})

	useEffect(() => {
		if (form.control._defaultValues.image) {
			setInitialPreview(URL.createObjectURL(form.control._defaultValues.image))
		}
	}, [form.control._defaultValues.image])

	const file = form.watch('image')
	useEffect(() => {
		if (file) {
			setPreview(URL.createObjectURL(file))
		}
	}, [file])

	return (
		<FormField
			name="image"
			control={form.control}
			render={({
				field: { name },
				fieldState: { isDirty, invalid: isError, error },
				formState: { disabled },
			}) => (
				<FormItem className="w-full">
					<FormControl>
						<>
							<div className="flex w-full justify-between items-center">
								<FormLabelError
									htmlFor={name}
									label={
										<div className="space-x-2 items-center flex">
											<span>{`Image${required ? ' (Required)' : ''}`}</span>
											{isDirty && (
												<div className="bg-blue-500 rounded-full w-3 h-3 mt-1" />
											)}
											{isError && (
												<div className="bg-red-500 rounded-full w-3 h-3 mt-1" />
											)}
										</div>
									}
									error={error?.message}
								/>
								{!required && form.watch('image') && (
									<XIcon
										className="text-red-500 cursor-pointer"
										width={20}
										height={20}
										onClick={deleteImage}
									/>
								)}
							</div>
							<div
								className={classNames(
									'aspect-square w-full bg-secondary rounded-lg overflow-hidden',
									disabled ? 'cursor-not-allowed' : 'cursor-pointer',
									className,
								)}
							>
								<div
									{...getRootProps({ className: 'dropzone' })}
									className="w-full h-full justify-center items-center flex relative"
								>
									<input {...getInputProps()} />
									<p className="absolute">
										Drag and drop or click to select an image
									</p>
									<img
										className={classNames(
											'absolute object-contain w-full h-full',
											!preview && 'hidden',
										)}
										src={preview}
										alt=""
									/>
								</div>
							</div>
						</>
					</FormControl>
				</FormItem>
			)}
		/>
	)
}
