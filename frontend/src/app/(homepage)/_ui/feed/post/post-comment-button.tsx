import { MessageCircle } from "lucide-react";

interface PostCommentButton {
	post_id: number;
	comment_count: number;
}

export function PostCommentButton({
	post_id,
	comment_count,
}: PostCommentButton) {
	return (
		<div className="space-x-3 flex items-center">
			<MessageCircle width={28} height={28} className="cursor-pointer" />
			<span>{comment_count}</span>
		</div>
	);
}
