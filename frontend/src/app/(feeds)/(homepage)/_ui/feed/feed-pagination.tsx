'use client'

import { Pagination } from '@/app/_ui/pagination'
import { usePathname, useRouter, useSearchParams } from 'next/navigation'
import { useEffect, useMemo } from 'react'

interface FeedPagination {
	page: number
	hasMore: boolean
}

export function FeedPagination({ page, hasMore }: FeedPagination) {
	const { push, prefetch } = useRouter()
	const searchParams = useSearchParams()
	const pathname = usePathname()

	const urlWithoutPage = useMemo(() => {
		const params = new URLSearchParams(searchParams)

		params.delete('page')

		return `${pathname}${params.toString()}`
	}, [pathname, searchParams])

	useEffect(() => {
		if (hasMore) {
			prefetch(`${urlWithoutPage}?page=${page + 1}`)
		}
	}, [hasMore, urlWithoutPage, page, prefetch])

	return (
		<Pagination
			page={page}
			canGoPrevious={page > 1}
			canGoNext={hasMore}
			onNext={() => {
				push(`${urlWithoutPage}?page=${page + 1}`)
			}}
			onPrevious={() => {
				push(`${urlWithoutPage}?page=${page - 1}`)
			}}
		/>
	)
}
