import { type VariantProps, cva } from 'class-variance-authority'
import * as React from 'react'
import TextareaAutosize from 'react-textarea-autosize'

import classNames from 'classnames'
import type { IconComponent } from '../types'
import { buttonIconVariants } from './button'

const textAreaVariants = cva(
	'truncate appearance-none dark:text-slate-50 text-gray-900 w-full !ring-0 !outline-none whitespace-pre-wrap',
	{
		variants: {
			size: {
				sm: 'min-h-[36px] h-[36px] py-1',
				default: 'min-h-[40px] h-[40px] py-2',
			},
			variant: {
				default:
					'border-0 flex items-center px-3 rounded-lg font-medium block bg-secondary group-hover:bg-muted group-focus:bg-accent',
				naked: 'border-0 bg-transparent',
				outline:
					'bg-secondary flex items-center px-3 rounded-lg font-medium block border border-accent group-hover:border-black/20 group-focus:border-black/30 hover:border-black/30 focus-within:border-black/30 dark:group-hover:border-white/20 dark:group-focus:border-white/30 dark:hover:border-white/30 dark:focus-within:border-white/30',
			},
			isError: {
				yes: 'bg-red/10 text-red',
				no: '',
			},
			hasIcon: {
				yes: 'pl-[40px]',
				no: '',
			},
			hasUnit: {
				yes: 'rounded-r-none !border-r-0',
				no: '',
			},
		},
		defaultVariants: {
			variant: 'default',
			hasIcon: 'no',
			hasUnit: 'no',
			size: 'default',
			isError: 'no',
		},
	},
)

interface TextAreaBaseProps
	extends Omit<React.InputHTMLAttributes<HTMLTextAreaElement>, 'size'>,
		Omit<VariantProps<typeof textAreaVariants>, 'isError'> {
	isError?: boolean
	id?: string
	icon?: IconComponent
	iconProps?: Omit<React.ComponentProps<'svg'>, 'width' | 'height'>
	unit?: string
}

interface TextAreaDynamicProps {
	onValueChange?(val: string): void
}

export type TextAreaProps = TextAreaBaseProps & TextAreaDynamicProps

const Component = (
	{
		icon: Icon,
		iconProps,
		unit,
		variant,
		className,
		type,
		onChange,
		size,
		onValueChange,
		isError,
		style: _style,
		...props
	}: TextAreaProps,
	ref: React.ForwardedRef<HTMLTextAreaElement>,
) => {
	const _onChange: React.InputHTMLAttributes<HTMLTextAreaElement>['onChange'] =
		(e) => {
			const nextUserInput = e.target.value
			if (typeof nextUserInput === 'undefined') {
				return
			}

			onValueChange?.(nextUserInput)

			if (onChange) {
				onChange(e)
			}
		}

	return (
		<div className="group relative flex items-center justify-between w-full">
			{Icon ? (
				<Icon
					{...iconProps}
					className={buttonIconVariants({
						className: classNames(
							'text-muted-foreground absolute left-3',
							iconProps?.className,
						),
					})}
				/>
			) : null}
			<TextareaAutosize
				onChange={_onChange}
				className={textAreaVariants({
					isError: isError ? 'yes' : 'no',
					variant,
					hasIcon: Icon ? 'yes' : 'no',
					hasUnit: unit ? 'yes' : 'no',
					className: classNames(
						className,
						'flex-grow flex-1 !outline-none !ring-0',
					),
				})}
				ref={ref}
				autoCorrect="off"
				autoCapitalize="none"
				spellCheck="false"
				autoComplete="off"
				{...props}
			/>
			{unit ? (
				<div
					className={textAreaVariants({
						isError: isError ? 'yes' : 'no',
						variant,
						size,
						className: 'text-muted-foreground rounded-l-none !w-[unset]',
					})}
				>
					{unit}
				</div>
			) : null}
		</div>
	)
}

const TextArea = React.forwardRef(Component)
TextArea.displayName = 'TextField'

const TextAreaDescription = React.forwardRef<
	HTMLParagraphElement,
	React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => {
	return (
		<p
			ref={ref}
			className={classNames('text-sm text-muted-foreground', className)}
			{...props}
		/>
	)
})
TextAreaDescription.displayName = 'TextFieldDescription'

export {
	TextArea,
	type TextAreaBaseProps,
	TextAreaDescription,
	textAreaVariants,
}
