import { PostDialog } from '@/app/_ui/post/post-dialog/post-dialog'
import Image from 'next/image'
import { FeedSearchProvider } from './feed-search/feed-search-provider'
import { FeedSortDropdown } from './feed-sort'
import { Suspense } from 'react'
import { getTypedSearchParams } from '@/app/_lib/typed-search-params/get-typed-search-params'
import { feedSearchSchema } from './feed-search/feed-search-schema'
import { dummyPosts } from '@/app/_types/post'
import { fetchGroupByUsername } from '@/app/(feeds)/group/_lib/fetch-groups-by-username'
import { fetchWallById } from '../../_lib/fetch-wall-by-id'
import { fetchUserByUsername } from '@/app/_lib/user/fetch-user'
import { fetchWallByTag } from '../../_lib/fetch-wall-by-tag'
import type { FeedFilters } from '../../_lib/filters'

type MiniatureFeed = (
	| {
			username: string
	  }
	| {
			groupname: string
	  }
	| {
			tag: string
	  }
) & {
	searchParams: Record<string, string>
}

export async function MiniatureFeed(props: MiniatureFeed) {
	return (
		<Suspense fallback={null}>
			<_MiniatureFeed {...props} />
		</Suspense>
	)
}

async function fetchPosts(props: MiniatureFeed, filters: FeedFilters) {
	if ('username' in props) {
		const username = props.username

		const user = await fetchUserByUsername(username)

		const posts = await fetchWallById(user.wallId, filters)

		return { posts }
	}

	if ('groupname' in props) {
		const groupname = props.groupname

		const group = await fetchGroupByUsername(groupname)
		const groupModeratorId = group.admin.id

		const posts = await fetchWallById(group.wallId, filters)

		return { posts, groupModeratorId }
	}

	if ('tag' in props) {
		const tag = props.tag

		const posts = await fetchWallByTag(tag, filters)

		return { posts }
	}

	throw new Error('Invalid props')
}

async function _MiniatureFeed(props: MiniatureFeed) {
	const filters = getTypedSearchParams(feedSearchSchema, props.searchParams)

	const { posts, groupModeratorId } = await fetchPosts(props, filters)

	return (
		<FeedSearchProvider>
			<div className="w-full flex justify-end">
				<div className="w-1/3">
					<FeedSortDropdown />
				</div>
			</div>
			<div className="grid grid-cols-3 gap-3">
				{posts.map((post) => (
					<div key={post.id} className="w-full h-full relative aspect-square">
						<PostDialog post={post} groupModeratorId={groupModeratorId}>
							<Image
								src={post.image.src}
								fill
								unoptimized
								alt="image"
								className="object-contain"
							/>
						</PostDialog>
					</div>
				))}
			</div>
		</FeedSearchProvider>
	)
}
