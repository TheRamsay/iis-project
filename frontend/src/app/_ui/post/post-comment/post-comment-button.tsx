import type { Post } from '@/app/post/_lib/fetch-post'
import { MessageCircle } from 'lucide-react'

interface PostCommentButton {
	post: Pick<Post, 'comments'>
}

export function PostCommentButton({ post }: PostCommentButton) {
	return (
		<div className="space-x-3 flex items-center">
			<MessageCircle
				width={28}
				height={28}
				className="cursor-pointer hover:text-accent-foreground"
			/>
			<span>{post.comments.length}</span>
		</div>
	)
}
