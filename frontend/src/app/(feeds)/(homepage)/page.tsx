import { Feed } from './_ui/feed/feed'
import { FeedSortDropdown } from '../_ui/feed/feed-sort'
import { FeedSearchProvider } from '../_ui/feed/feed-search/feed-search-provider'
import { getTypedSearchParams } from '../../_lib/typed-search-params/get-typed-search-params'
import { feedSearchSchema } from '../_ui/feed/feed-search/feed-search-schema'
import { FeedPagination } from './_ui/feed/feed-pagination'
import { fetchMyFeed } from '../_lib/fetch-my-feed'

const pageSize = 10

export default async function Page({
	searchParams,
}: { searchParams: Record<string, string> }) {
	const { page, sorting } = getTypedSearchParams(feedSearchSchema, searchParams)

	const feed = await fetchMyFeed({
		pagination: {
			pageIndex: page,
			pageSize,
		},
		sorting,
	})

	return (
		<FeedSearchProvider>
			<div className="w-full flex justify-end">
				<div className="w-1/3">
					<FeedSortDropdown />
				</div>
			</div>
			<div className="space-y-8">
				<Feed data={feed} />
				<FeedPagination page={page} hasMore={feed.length === pageSize} />
			</div>
		</FeedSearchProvider>
	)
}
