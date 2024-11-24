import { SkeletonCircle, SkeletonText } from '@/components/components/skeleton'
import Image from 'next/image'
import Link from 'next/link'
import classNames from 'classnames'
import type { Post } from '@/app/post/_lib/fetch-post'
import { PostCommentDeleteButton } from './post-comment-delete-button'
import type { Comment as CommentType } from '@/app/_types/comments'

interface PostComments {
	post: Pick<Post, 'comments' | 'id'> & {
		user: Pick<Post['user'], 'id'>
	}
	size: 'small' | 'full'
}

export function PostComments({ post, size }: PostComments) {
	const data = post.comments

	return (
		<div className="space-y-2">
			{data.map((comment) => (
				<Comment post={post} key={comment.id} comment={comment} size={size} />
			))}
		</div>
	)
}

type CommentLoading = {
	post: undefined
	comment: undefined
	isLoading: true
	size: 'small' | 'full'
}

type CommentLoaded = {
	post: Pick<Post, 'id'> & {
		user: Pick<Post['user'], 'id'>
	}
	comment: CommentType
	isLoading?: undefined | false
	size: 'small' | 'full'
}

type CommentProps = CommentLoading | CommentLoaded

function Comment({ post, comment, isLoading, size }: CommentProps) {
	const circleSize = size === 'small' ? 20 : 48

	const Shell = ({ children }: { children: React.ReactNode }) => (
		<div
			className={classNames(
				'flex flex-row items-center',
				size === 'small' && 'space-x-2',
				size === 'full' && 'space-x-4',
			)}
		>
			{children}
		</div>
	)

	if (isLoading) {
		return (
			<Shell>
				<SkeletonCircle radius={circleSize} />
				<span className="w-full flex flex-row space-x-2">
					<div className="w-20">
						<SkeletonText fontSize="sm" />
					</div>
					<SkeletonText fontSize="sm" className="w-full" />
				</span>
			</Shell>
		)
	}

	return (
		<div className="flex justify-between items-center space-x-4">
			<Shell>
				<Image
					unoptimized={true}
					src={comment.user.avatar}
					alt="avatar"
					width={circleSize}
					height={circleSize}
					className="rounded-full flex-grow-0 object-contain"
				/>
				<p className="space-x-1 text-sm [word-break:break-word]">
					<Link
						href={`/profile/${comment.user.username}`}
						className="float-left flex flex-row space-x-2"
					>
						<span className="font-semibold whitespace-nowrap">
							{comment.user.username}:
						</span>
					</Link>
					<span>{comment.content}</span>
				</p>
			</Shell>
			<PostCommentDeleteButton size="small" post={post} comment={comment} />
		</div>
	)
}
