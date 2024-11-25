import { SkeletonCircle, SkeletonText } from '@/components/components/skeleton'
import Image from 'next/image'
import Link from 'next/link'
import classNames from 'classnames'
import { fetchPost, type Post } from '@/app/post/_lib/fetch-post'
import { PostCommentDeleteButton } from './post-comment-delete-button'
import type { Comment as CommentType } from '@/app/_types/comments'
import { unstable_cache } from 'next/cache'
import { Suspense } from 'react'
import { Avatar } from '../../avatar'

interface PostComments {
	post: Pick<Post, 'comments' | 'id'> & {
		user: Pick<Post['user'], 'id'>
	}
	showCount?: number
	size: 'small' | 'full'
}

export async function PostComments({ post, showCount, size }: PostComments) {
	return (
		<Suspense
			fallback={
				<div className="space-y-2">
					<Comment isLoading size={size} />
					<Comment isLoading size={size} />
					<Comment isLoading size={size} />
				</div>
			}
		>
			<_PostComments post={post} showCount={showCount} size={size} />
		</Suspense>
	)
}

async function _PostComments({ post: { id }, showCount, size }: PostComments) {
	const post = await fetchPost(id)

	const comments = showCount ? post.comments.slice(0, showCount) : post.comments

	return (
		<div className="space-y-2">
			{comments.map((comment) => (
				<Comment post={post} key={comment.id} comment={comment} size={size} />
			))}
		</div>
	)
}

type CommentLoading = {
	post?: undefined
	comment?: undefined
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
				<Avatar
					name={comment.user.username}
					unoptimized={true}
					src={comment.user.avatar.src}
					alt="avatar"
					className="rounded-full"
					size={32}
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
