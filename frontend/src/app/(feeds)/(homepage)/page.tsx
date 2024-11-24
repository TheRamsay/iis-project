import { Feed } from './_ui/feed/feed'
import { getSession } from '../../_lib/auth/get-session'
import { FeedSortDropdown } from '../../_ui/feed/feed-sort'
import { FeedSearchProvider } from '../../_ui/feed/feed-search/feed-search-provider'
import { getTypedSearchParams } from '../../_lib/typed-search-params/get-typed-search-params'
import { feedSearchSchema } from '../../_ui/feed/feed-search/feed-search-schema'
import { backendFetch } from '@/app/_lib/backend-fetch'
import { FeedPagination } from './_ui/feed/feed-pagination'

const pageSize = 10

export default async function Page({
	searchParams,
}: { searchParams: Record<string, string> }) {
	const session = await getSession()
	const filters = getTypedSearchParams(feedSearchSchema, searchParams)

	const response = await backendFetch(
		`/api/walls/feed?offset=${pageSize * filters.page}&limit=${pageSize}&sort=${filters.sorting}`,
	)

	if (!response.ok) {
		return <div>Failed to load feed</div>
	}

	const { posts: feed } = await response.json()

	// TODO: Format

	return (
		<FeedSearchProvider>
			<div className="w-full flex justify-end">
				<div className="w-1/3">
					<FeedSortDropdown />
				</div>
			</div>
			<div className="space-y-8">
				<Feed data={[]} />
				<FeedPagination
					page={filters.page}
					hasMore={feed.length === pageSize}
				/>
			</div>
		</FeedSearchProvider>
	)
}
