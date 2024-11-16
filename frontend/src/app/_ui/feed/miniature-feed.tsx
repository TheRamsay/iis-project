import { PostDialog } from '@/app/_ui/post/post-dialog'
import Image from 'next/image'
import { FeedSearchProvider } from './feed-search/feed-search-provider'
import { FeedSortDropdown } from './feed-sort'
import { Suspense } from 'react'
import { getTypedSearchParams } from '@/app/_lib/typed-search-params/get-typed-search-params'
import { feedSearchSchema } from './feed-search/feed-search-schema'

type MiniatureFeed = (
	| {
			username: string
	  }
	| {
			groupname: string
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

async function _MiniatureFeed(props: MiniatureFeed) {
	const type: 'group' | 'user' = 'groupname' in props ? 'group' : 'user'
	const name = 'groupname' in props ? props.groupname : props.username
	const filters = getTypedSearchParams(feedSearchSchema, props.searchParams)

	const posts = Array(10)
		.fill(0)
		.map((_, i) => ({
			id: i,
			image: {
				src: 'https://picsum.photos/256',
				width: 800,
				height: 800,
			},
			caption: 'This is a post',
			user: {
				id: '1',
				username: 'fitstagram',
				avatar: 'https://avatars.githubusercontent.com/u/7655549?v=4',
			},
			like_count: 0,
			comments: [
				{
					id: 1,
					user: {
						id: '1',
						username: 'fitstagram',
						avatar: 'https://avatars.githubusercontent.com/u/7655549?v=4',
					},
					content: 'This is a comment',
				},
				{
					id: 2,
					user: {
						id: '1',
						username: 'fitstagram',
						avatar: 'https://avatars.githubusercontent.com/u/7655549?v=4',
					},
					content: 'This is a comment',
				},
			],
		}))

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
						<PostDialog post={post}>
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
