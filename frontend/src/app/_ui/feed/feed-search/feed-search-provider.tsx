'use client'

import { parseArgs } from '@/app/_lib/parse-args'
import { useTypedSearchParams } from '@/app/_lib/typed-search-params/use-typed-search-params'
import { useRouter } from 'next/navigation'
import {
	type Dispatch,
	type FC,
	type ReactNode,
	type SetStateAction,
	createContext,
	useContext,
	useMemo,
} from 'react'
import { SortOption } from './types'
import { feedSearchSchema } from './feed-search-schema'
import type { z } from 'zod'

type FilterContext = z.TypeOf<typeof feedSearchSchema>

const FilterContext = createContext<FilterContext | undefined>(undefined)

export type FeedSearch = Omit<FilterContext, 'setFilters'>

interface FeedSearchProvider {
	children?: ReactNode
}

export const FeedSearchProvider: FC<FeedSearchProvider> = ({ children }) => {
	const urlFilters = useTypedSearchParams(feedSearchSchema.partial())
	const { sorting } = urlFilters

	return (
		<FilterContext.Provider
			value={useMemo(
				() => ({
					sorting: sorting || SortOption.New,
				}),
				[sorting],
			)}
		>
			{children}
		</FilterContext.Provider>
	)
}

export const useFeedSearch = () => {
	const context = useContext(FilterContext)
	if (!context) {
		throw new Error('Hook can only be used inside Filter Context')
	}

	return context
}

export const useSetFeedSearch = () => {
	const { push } = useRouter()
	const urlFilters = useTypedSearchParams(feedSearchSchema)

	const setFilters: Dispatch<SetStateAction<typeof urlFilters>> = (filters) => {
		if (typeof filters === 'function') {
			void push(parseArgs(filters(urlFilters)), { scroll: false })
		} else {
			void push(parseArgs(filters), { scroll: false })
		}
	}

	return setFilters
}
