import classNames from 'classnames'
import { ChevronLeftIcon, ChevronRightIcon } from 'lucide-react'

interface Pagination {
	page: number
	canGoPrevious: boolean
	canGoNext: boolean
	onNext: () => void
	onPrevious: () => void
}

export function Pagination({
	page,
	canGoPrevious,
	canGoNext,
	onNext,
	onPrevious,
}: Pagination) {
	return (
		<div className="w-full justify-between flex items-center">
			<div>Page: {page}</div>
			<div className="flex space-x-2">
				<div
					onClick={onPrevious}
					className={classNames(
						canGoPrevious ? 'cursor-pointer' : 'opacity-50 pointer-events-none',
					)}
				>
					<ChevronLeftIcon className="h-8 w-8" />
				</div>
				<div
					onClick={onNext}
					className={classNames(
						canGoNext
							? 'cursor-pointer'
							: 'opacity-50 opacity-50 pointer-events-none',
					)}
				>
					<ChevronRightIcon className="h-8 w-8" />
				</div>
			</div>
		</div>
	)
}
