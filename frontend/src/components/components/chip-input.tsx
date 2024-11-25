'use client'

import type { VariantProps } from 'class-variance-authority'
import classNames from 'classnames'
import { useCallback, forwardRef } from 'react'
import {
	type FC,
	useEffect,
	useMemo,
	useRef,
	useState,
	useTransition,
} from 'react'

import type { IconComponent } from '../types'
import { buttonIconVariants } from './button'
import { Chip, type chipVariants } from './chip'
import { textFieldVariants } from './text-field'
import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from './tooltip'
import { useOnClickOutside } from '@/app/_ui/header/use-on-click-outside'

export type ChipInputRootProps = React.InputHTMLAttributes<HTMLDivElement>

const ChipInputRoot = forwardRef<HTMLDivElement, ChipInputRootProps>(
	({ ...props }, ref) => {
		return <div ref={ref} className="flex gap-2 items-center" {...props} />
	},
)
ChipInputRoot.displayName = 'ChipInputRoot'

interface ChipInputProps
	extends Omit<React.HTMLAttributes<HTMLInputElement>, 'size'>,
		Omit<VariantProps<typeof chipVariants>, 'variant'>,
		VariantProps<typeof textFieldVariants> {
	icon?: IconComponent
	iconProps?: Omit<React.ComponentProps<'svg'>, 'width' | 'height'>
	onValueChange(values: string[]): void
	values: string[]
	mutateValue?(string: string): string
	delimiters?: string[]
	maxValues?: number
	placeholder?: string
}

function codeTranslator(code: string): string {
	if (code === 'Enter') return '\n'
	return code
}

const ChipInput: FC<ChipInputProps> = ({
	className,
	icon: Icon,
	iconProps,
	size,
	values,
	variant,
	onValueChange,
	delimiters: _delimiters = [',', ';', ':', ' ', 'Enter', 'Tab'],
	mutateValue,
	maxValues,
	...props
}) => {
	const ref = useRef<HTMLInputElement>(null)
	const [fullTags, setFullTags] = useState(values)
	const [partialTag, setPartialTag] = useState('')
	const [_pending, startTransition] = useTransition()

	const delimiters = useMemo(
		() => _delimiters.map(codeTranslator),
		[_delimiters],
	)

	const addFullTag = useCallback(() => {
		if (partialTag === '') return

		setFullTags((prev) => {
			const newTags = [...prev, partialTag]
			return newTags
		})

		setPartialTag('')
	}, [partialTag])

	useOnClickOutside(ref, addFullTag)

	const removeTag = useCallback((index: number) => {
		setFullTags((prev) => {
			const newTags = [...prev.slice(0, index), ...prev.slice(index + 1)]
			return newTags
		})
	}, [])

	const onKeyDown = useCallback(
		(e: React.KeyboardEvent<HTMLInputElement>) => {
			if (delimiters.includes(codeTranslator(e.key))) {
				addFullTag()
			} else if (e.code === 'Backspace') {
				if (partialTag === '') {
					removeTag(fullTags.length - 1)
				} else {
					setPartialTag((prev) => prev.slice(0, -1))
				}
			}
		},
		[addFullTag, partialTag, fullTags, removeTag, delimiters],
	)

	const onChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setPartialTag(e.target.value)
	}, [])

	const allTags = useMemo(() => {
		if (partialTag === '') return fullTags

		return [...fullTags, partialTag]
	}, [fullTags, partialTag])

	useEffect(() => {
		onValueChange(allTags)
	}, [onValueChange, allTags])

	return (
		<ChipInputRoot
			className={textFieldVariants({
				variant,
				size,
				className: 'relative gap-2 flex-wrap !h-[unset]',
			})}
			ref={ref}
		>
			{Icon ? <Icon {...iconProps} className={buttonIconVariants()} /> : null}
			{fullTags.length > 0
				? fullTags.map((value, i) => (
						<TooltipProvider key={i}>
							<Tooltip>
								<TooltipTrigger asChild>
									<Chip onClose={() => removeTag(i)} variant="secondary">
										{mutateValue ? mutateValue(value) : value}
									</Chip>
								</TooltipTrigger>
								<TooltipContent>
									<p>{value}</p>
								</TooltipContent>
							</Tooltip>
						</TooltipProvider>
					))
				: null}
			{(maxValues ? fullTags.length < maxValues : true) ? (
				<input
					onKeyDown={onKeyDown}
					className={classNames(
						className,
						fullTags.length > 0 && 'placeholder:text-transparent',
						'flex flex-grow bg-transparent truncate !outline-none !ring-0',
					)}
					onChange={onChange}
					value={partialTag}
					{...props}
				/>
			) : null}
		</ChipInputRoot>
	)
}

export { ChipInput }
