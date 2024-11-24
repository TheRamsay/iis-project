import { PostDialog } from '@/app/_ui/post/post-dialog/post-dialog'
import Image from 'next/image'
import { FeedSearchProvider } from './feed-search/feed-search-provider'
import { FeedSortDropdown } from './feed-sort'
import { Suspense } from 'react'
import { getTypedSearchParams } from '@/app/_lib/typed-search-params/get-typed-search-params'
import { feedSearchSchema } from './feed-search/feed-search-schema'
import { dummyPosts } from '@/app/_types/post'
import { fetchGroupByUsername } from '@/app/(feeds)/group/_lib/fetch-groups-by-username'

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

async function fetchPosts(
	props: MiniatureFeed,
	filters: ReturnType<typeof getTypedSearchParams<typeof feedSearchSchema>>,
) {
	if ('username' in props) {
		// TODO: endpoint
		const username = props.username
		const posts = dummyPosts
		return { posts }
	}

	if ('groupname' in props) {
		// TODO: endpoint
		const groupname = props.groupname
		const posts = dummyPosts

		const group = await fetchGroupByUsername(groupname)
		const groupModeratorId = group.admin.id

		return { posts, groupModeratorId }
	}

	if ('tag' in props) {
		// TODO: endpoint
		const tag = props.tag
		const posts = dummyPosts
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
