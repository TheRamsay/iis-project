import { Feed } from './_ui/feed/feed'
import { getSession } from '../_lib/auth/get-session'
import { FeedSortDropdown } from '../_ui/feed/feed-sort'
import { FeedSearchProvider } from '../_ui/feed/feed-search/feed-search-provider'
import { getTypedSearchParams } from '../_lib/typed-search-params/get-typed-search-params'
import { feedSearchSchema } from '../_ui/feed/feed-search/feed-search-schema'

export default async function Page({
	searchParams,
}: { searchParams: Record<string, string> }) {
	const session = await getSession()
	const filters = getTypedSearchParams(feedSearchSchema, searchParams)

	const entry = {
		id: 1,
		image: {
			src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
			width: 128,
			height: 128,
		},
		caption: 'This is a post',
		user: {
			id: '1',
			username: 'fitstagram',
			avatar: {
				src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
				width: 128,
				height: 128,
			},
		},
		like_count: 0,
		comments: [
			{
				id: 1,
				user: {
					id: '1',
					username: 'fitstagram',
					avatar: {
						src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
						width: 128,
						height: 128,
					},
				},
				content: 'This is a comment',
			},
			{
				id: 2,
				user: {
					id: '1',
					username: 'fitstagram',
					avatar: {
						src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
						width: 128,
						height: 128,
					},
				},
				content: 'This is a comment',
			},
		],
	}

	const feed = [entry, { ...entry, id: 2 }]

	return (
		<FeedSearchProvider>
			<div className="w-full flex justify-end">
				<div className="w-1/3">
					<FeedSortDropdown />
				</div>
			</div>
			<Feed data={feed} />
		</FeedSearchProvider>
	)
}
