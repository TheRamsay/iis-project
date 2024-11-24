import { MessageCircle } from 'lucide-react'

interface PostCommentButton {
	postId: number
	commentCount: number
}

export function PostCommentButton({ commentCount }: PostCommentButton) {
	return (
		<div className="space-x-3 flex items-center">
			<MessageCircle
				width={28}
				height={28}
				className="cursor-pointer hover:text-accent-foreground"
			/>
			<span>{commentCount}</span>
		</div>
	)
}
