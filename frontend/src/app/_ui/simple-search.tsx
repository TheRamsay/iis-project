'use client'

import { useOnClickOutside } from '@/app/_ui/header/use-on-click-outside'
import classNames from 'classnames'
import { SearchIcon, XIcon } from 'lucide-react'
import { useRef, useState } from 'react'

interface SimpleSearch<Data extends unknown[]> {
	query: string
	setQuery: (query: string) => void
	placeholder: string
	data: Data | undefined
	isError: boolean
	isLoading: boolean
	dataRenderer: (data: SimpleSearchDataProps<Data>) => JSX.Element
}

export type SimpleSearchDataProps<Data extends unknown[]> = Pick<
	SimpleSearch<Data>,
	'data' | 'isLoading' | 'isError'
>

export function SimpleSearch<Data extends unknown[]>({
	query,
	setQuery,
	placeholder,
	data,
	isError,
	isLoading,
	dataRenderer,
}: SimpleSearch<Data>) {
	const [open, setOpen] = useState<boolean>(false)
	const ref = useRef<HTMLDivElement>(null)
	useOnClickOutside(ref, () => setOpen(false))

	return (
		<div className="flex flex-col gap-3 relative justify-center w-full">
			<div className="z-10 flex w-full gap-4">
				<div
					ref={ref}
					onFocus={() => setOpen(true)}
					className={classNames('rounded-lg w-full border border-accent')}
				>
					<div className="flex items-center gap-2 pl-3 pr-3 h-10 w-full">
						<div className="flex gap-4 items-center w-full">
							<div className="w-4 h-4">
								<SearchIcon
									width={16}
									height={16}
									className="dark:text-gray-300 text-neutral-950"
								/>
							</div>
							<input
								value={query}
								onChange={(e) => setQuery(e.target.value)}
								placeholder={placeholder}
								className={classNames(
									'w-full dark:placeholder:accent-foreground flex-grow-0',
									'p-0 bg-transparent border-none focus:outline-none focus:ring-0 w-full truncate font-medium text-left text-base md:text-sm placeholder:font-normal',
								)}
							/>
						</div>
						{query && (
							<XIcon
								onClick={() => setQuery('')}
								className="w-6 h-6 cursor-pointer dark:text-gray-500 text-neutral-950"
							/>
						)}
					</div>
					<div
						className={classNames(
							open
								? 'max-h-[335px] py-2 border-b border-l border-r -ml-[0.5px] scroll'
								: 'max-h-[0px]',
							'z-[100]  rounded-b-xl flex flex-col gap-2 overflow-hidden transition-all absolute w-full -mt-1',
							'bg-background border-accent',
						)}
					>
						{dataRenderer({ data, isError, isLoading })}
					</div>
				</div>
			</div>
		</div>
	)
}
