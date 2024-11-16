'use client'

import classNames from 'classnames'
import { useEffect, useState } from 'react'
import Dropzone, { useDropzone } from 'react-dropzone'

interface ImageDropzoneProps {
	file: globalThis.File | null
	setFile: (file: globalThis.File) => void
}

export function ImageDropzone({ file, setFile }: ImageDropzoneProps) {
	const [preview, setPreview] = useState<string | undefined>()

	const { getRootProps, getInputProps } = useDropzone({
		accept: {
			'image/*': [],
		},
		maxFiles: 1,
		multiple: false,
		onDrop: ([file]) => {
			setFile(file)
			setPreview(URL.createObjectURL(file))
		},
	})

	useEffect(() => {
		return () => {
			preview && URL.revokeObjectURL(preview)
		}
	}, [preview])

	return (
		<div className="cursor-pointer aspect-square w-full bg-secondary rounded-lg overflow-hidden">
			<div
				{...getRootProps({ className: 'dropzone' })}
				className="w-full h-full justify-center items-center flex relative"
			>
				<input {...getInputProps()} />
				<p className="absolute">Drag and drop or click to select an image</p>
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
	)
}
