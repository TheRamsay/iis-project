'use client'

import { SkeletonCircle, SkeletonText } from '@/components/components'
import { useQuery } from '@tanstack/react-query'
import Image from 'next/image'
import Link from 'next/link'
import { useCallback } from 'react'
import { PostDeleteButton } from '../post-delete-button'
import classNames from 'classnames'

interface PostComments {
	post: {
		id: number
		comments?:
			| {
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
			| undefined
	}
	size: 'small' | 'full'
}

const undefinedComments = [undefined, undefined, undefined]

export function PostComments({ post, size }: PostComments) {
	const { data: fetchedComments, isLoading } = useQuery({
		queryKey: ['comments', post.id],
		queryFn: async () => {
			// TODO: endpoint
			await new Promise((resolve) => setTimeout(resolve, 1000))

			return [
				{
					id: 1,
					user: {
						id: '1',
						username: 'user1',
						avatar: {
							src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
							width: 128,
							height: 128,
						},
					},
					content:
						'comment1co mment1comme nt1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1',
				},
				{
					id: 2,
					user: {
						id: '2',
						username: 'user1',
						avatar: {
							src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
							width: 128,
							height: 128,
						},
					},
					content:
						'comment1co mment1comme nt1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1comment1',
				},
			]
		},
		enabled: !post.comments,
	})

	if (isLoading) {
		return (
			<div className="space-y-2">
				{undefinedComments.map((_, i) => (
					<Comment key={i} comment={undefined} isLoading={true} size={size} />
				))}
			</div>
		)
	}

	const data = fetchedComments || post.comments

	if (!data) {
		return null
	}

	return (
		<div className="space-y-2">
			{data.map((comment) => (
				<Comment key={comment.id} comment={comment} size={size} />
			))}
		</div>
	)
}

type CommentLoading = {
	comment: undefined
	isLoading: true
	size: 'small' | 'full'
}

type CommentLoaded = {
	comment: NonNullable<PostComments['post']['comments']>[number]
	isLoading?: undefined | false
	size: 'small' | 'full'
}

type CommentProps = CommentLoading | CommentLoaded

function Comment({ comment, isLoading, size }: CommentProps) {
	const circleSize = size === 'small' ? 20 : 48

	const Shell = useCallback(
		({ children }: { children: React.ReactNode }) => (
			<div
				className={classNames(
					'flex flex-row items-center',
					size === 'small' && 'space-x-2',
					size === 'full' && 'space-x-4',
				)}
			>
				{children}
			</div>
		),
		[size],
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
			<PostDeleteButton
				size="small"
				postId={comment.id}
				postAuthorId={comment.user.id}
			/>
		</div>
	)
}
