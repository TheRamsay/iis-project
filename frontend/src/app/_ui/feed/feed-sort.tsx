'use client'

import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from '@/components/components/select'
import {
	useFeedSearch,
	useSetFeedSearch,
} from './feed-search/feed-search-provider'
import { useCallback } from 'react'
import type { SortOption } from './feed-search/types'

export function FeedSortDropdown() {
	const { sorting } = useFeedSearch()
	const setFilters = useSetFeedSearch()

	const onChange = useCallback(
		(value: SortOption) => {
			setFilters((prev) => ({
				...prev,
				sorting: value as SortOption,
			}))
		},
		[setFilters],
	)

	const firstCap = sorting.charAt(0).toUpperCase() + sorting.slice(1)

	return (
		<Select value={sorting} onValueChange={onChange}>
			<SelectTrigger>{firstCap}</SelectTrigger>
			<SelectContent>
				<SelectItem value="new">New</SelectItem>
				<SelectItem value="top">Top</SelectItem>
			</SelectContent>
		</Select>
	)
}
