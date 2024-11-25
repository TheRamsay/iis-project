import Image from 'next/image'
import { UserAvatarName } from '../../user/user-avatar-name'
import { DialogClose } from '@/components/components/dialog'
import { XIcon } from 'lucide-react'
import Link from 'next/link'
import { PostComments } from '../post-comment/post-comments'
import { PostLikeButton } from '../post-like-button'
import { PostEditButton } from '../post-edit-button'
import { PostDeleteButton } from '../post-delete-button'
import { PostCommentAdd } from '../post-comment/post-comment-add'
import type { Post } from '@/app/post/_lib/fetch-post'

type PostDialogContent = {
	post: Post
	group?: {
		id: string
		moderatorId: string
	}
	dialog?: boolean
}

export function PostDialogContent({
	post,
	group,
	dialog = true,
}: PostDialogContent) {
	return (
		<div className="flex flex-row w-full h-full">
			<div className="relative border-accent border-r h-full w-full">
				<Image
					unoptimized
					src={post.image.src}
					alt="image"
					width={512}
					height={512}
					className="h-full w-full object-contain"
				/>
			</div>
			<div className="flex flex-col justify-between w-[35%] min-w-[35%]">
				<div className="">
					<div className="p-4 border-b border-accent flex flex-row justify-between items-center">
						<UserAvatarName user={post.user} />
						{dialog && (
							<DialogClose>
								<XIcon width={24} height={24} />
							</DialogClose>
						)}
					</div>
					<div className="p-4 border-b border-accent space-y-1">
						<div className="space-y-2">
							<h2 className="text-xl font-medium">{post.title}</h2>
							{post.description && (
								<p className="text-sm">{post.description}</p>
							)}
						</div>
						{post.tags.length ? (
							<div className="space-x-1 text-blue-500 text-sm">
								{post.tags.map((tag) => (
									<Link key={tag} href={`/tag/${tag}`}>
										#{tag}
									</Link>
								))}
							</div>
						) : null}
					</div>
					<div className="p-4">
						<PostComments post={post} size="full" />
					</div>
				</div>
				<div>
					<div className="border-y border-accent p-4 flex justify-between">
						<div className="space-x-4">
							<PostLikeButton post={post} />
						</div>
						<div className="space-x-4 flex ">
							<PostEditButton post={post} />
							<PostDeleteButton post={post} group={group} />
						</div>
					</div>
					<div className="px-4 py-2">
						<PostCommentAdd post={post} />
					</div>
				</div>
			</div>
		</div>
	)
}
