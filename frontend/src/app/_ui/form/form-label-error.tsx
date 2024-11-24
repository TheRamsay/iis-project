import classNames from 'classnames'
import type React from 'react'

interface FormLabelError {
	htmlFor: string
	label: string | React.ReactNode
	error?: string | null
	className?: string
}

export function FormLabelError({
	htmlFor,
	label,
	error,
	className,
}: FormLabelError) {
	return (
		<>
			<div className={classNames('flex w-full justify-between', className)}>
				<label htmlFor={htmlFor}>{label}</label>
				<span className="text-red-500">{error}</span>
			</div>
		</>
	)
}
