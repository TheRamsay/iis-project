import Image from 'next/image'
import Link from 'next/link'
import { PostLikeButton } from '@/app/_ui/post/post-like-button'
import { PostCommentButton } from '@/app/_ui/post/post-comment/post-comment-button'
import { PostDialog } from '@/app/_ui/post/post-dialog'
import { PostComments } from '@/app/_ui/post/post-comment/post-comments'

interface Post {
	id: number
	image: {
		src: string
		width: number
		height: number
	}
	caption: string
	user: {
		username: string
		avatar: string
	}
	like_count: number
	comments: {
		id: number
		user: {
			username: string
			avatar: string
		}
		content: string
	}[]
}

export function Post(post: Post) {
	return (
		<div key={post.id} className="w-full flex flex-col space-y-3">
			<div>
				<div className="flex flex-row space-x-3 p-2 items-center w-full">
					<Image
						unoptimized={true}
						src={post.user.avatar}
						alt="avatar"
						width={32}
						height={32}
						className="rounded-full"
					/>
					<Link href={`/profile/${post.user.username}`}>
						<span>{post.user.username}</span>
					</Link>
				</div>
				<PostDialog post={post}>
					<div className="relative h-full w-full">
						<Image
							unoptimized={true}
							src={post.image}
							alt="image"
							width={post.image.width}
							height={post.image.height}
							className="h-auto w-full object-contain"
						/>
					</div>
				</PostDialog>
			</div>
			<div className="space-x-5 flex">
				<PostLikeButton postId={post.id} likeCount={post.like_count} />
				<PostCommentButton
					postId={post.id}
					commentCount={post.comments.length}
				/>
			</div>
			<PostComments post={post} size="small" />
		</div>
	)
}
