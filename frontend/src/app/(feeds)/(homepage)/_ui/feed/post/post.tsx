import Image from 'next/image'
import { PostLikeButton } from '@/app/_ui/post/post-like-button'
import { PostCommentButton } from '@/app/_ui/post/post-comment/post-comment-button'
import { PostDialog } from '@/app/_ui/post/post-dialog/post-dialog'
import { PostComments } from '@/app/_ui/post/post-comment/post-comments'
import { PostDeleteButton } from '@/app/_ui/post/post-delete-button'
import { UserAvatarName } from '@/app/_ui/user/user-avatar-name'
import type { Post as PostType } from '@/app/post/_lib/fetch-post'
import { PostShareButton } from '@/app/_ui/post/post-share-button'

export function Post(post: PostType) {
	return (
		<div key={post.id} className="w-full flex flex-col space-y-3">
			<div>
				<UserAvatarName user={post.user} />
				<PostDialog post={post}>
					<div className="relative h-full w-full">
						<Image
							unoptimized={true}
							src={post.image.src}
							alt="image"
							width={512}
							height={512}
							className="h-auto w-full object-contain"
						/>
					</div>
				</PostDialog>
			</div>
			<div className="flex justify-between">
				<div className="space-x-4 flex">
					<PostLikeButton post={post} />
					<PostCommentButton post={post} />
					<PostShareButton post={post} />
				</div>
				<div className="space-x-4 flex">
					<PostDeleteButton post={post} />
				</div>
			</div>
			<PostComments post={post} showCount={3} size="small" />
		</div>
	)
}
