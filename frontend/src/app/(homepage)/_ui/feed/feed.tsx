import { Post } from './post/post'

interface Feed {
	data: {
		id: number
		image: {
			src: string
			width: number
			height: number
		}
		caption: string
		user: {
			id: string
			username: string
			avatar: {
				src: string
				width: number
				height: number
			}
		}
		like_count: number
		comments: {
			id: number
			user: {
				id: string
				username: string
				avatar: {
					src: string
					width: number
					height: number
				}
			}
			content: string
		}[]
	}[]
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
