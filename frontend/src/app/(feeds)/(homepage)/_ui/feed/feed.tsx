import { Post } from './post/post'
import type { Post as PostType } from '@/app/post/_lib/fetch-post'

interface Feed {
	data: PostType[]
}

export function Feed({ data }: Feed) {
	return (
		<div className="divide-y-2 divide-accent">
			{data.map((post) => (
				<div key={post.id} className="py-6 first:pt-0">
					<Post {...post} />
				</div>
			))}
		</div>
	)
}
