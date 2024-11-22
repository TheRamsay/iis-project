'use client'

import classNames from 'classnames'
import { SearchIcon, XIcon } from 'lucide-react'
import { useRef, useState } from 'react'
import { useOnClickOutside } from './use-on-click-outside'
import { SkeletonText } from '@/components/components/skeleton'

// Inspired by https://github.com/sushi-labs/sushiswap/blob/feature/egn-677/apps/web/src/app/(cms)/faq/(root)/components/search-box.tsx

export function HeaderSearch() {
	const [query, setQuery] = useState<string>('')
	const [open, setOpen] = useState<boolean>(false)

	const ref = useRef<HTMLDivElement>(null)

	useOnClickOutside(ref, () => setOpen(false))

	const { data, isLoading, isError } = {
		data: [],
		isLoading: false,
		isError: false,
	}

	return (
		<div className="flex flex-col gap-3 relative justify-center">
			<div className="z-10 flex w-full gap-4 min-w-[400px]">
				<div
					ref={ref}
					onFocus={() => setOpen(true)}
					className={classNames('rounded-md w-full border border-accent')}
				>
					<div className="flex items-center gap-2 pl-3 pr-3 h-10">
						<div className="flex gap-4 items-center w-full">
							<div className="w-6 h-6">
								<SearchIcon
									width={24}
									height={24}
									className="dark:text-gray-300 text-neutral-950"
								/>
							</div>
							<input
								value={query}
								onChange={(e) => setQuery(e.target.value)}
								placeholder="Search profiles, groups, tags..."
								className={classNames(
									'w-full dark:placeholder:accent-foreground',
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
						<Results data={data} isError={isError} isLoading={isLoading} />
					</div>
				</div>
			</div>
		</div>
	)
}

interface Results {
	isLoading: boolean
	isError: boolean
	data: unknown[]
}

function Results({ data, isError, isLoading }: Results) {
	if (isError) {
		return (
			<div className="px-4 pt-4 pb-2 gap-2 flex justify-center w-full text-sm">
				An unexpected error has occured.
			</div>
		)
	}

	if (isLoading) {
		return (
			<div>
				<div className="mt-2 px-4">
					<SkeletonText />
				</div>
				<div className="px-4 py-2 gap-2">
					<SkeletonText fontSize="sm" />
					<SkeletonText fontSize="sm" />
					<SkeletonText fontSize="sm" />
				</div>
			</div>
		)
	}

	return (
		<div>
			<div className="font-medium mt-2 px-4 text-gray-400">Profiles</div>
			<div className="px-4 py-2 gap-2 text-sm">
				<div className="text-sm">data</div>
			</div>
		</div>
	)
}
