import { dummyPosts } from '@/app/_types/post'
import { PostDialogContent } from '@/app/_ui/post/post-dialog/post-dialog-content'

export default function Page({
	params: { postId },
}: { params: { postId: string } }) {
	// TODO: endpoint
	const post = {
		id: '1',
		title: 'Post 1',
		content: 'This is the',
	}

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Post</h1>
			<PostDialogContent post={dummyPosts[0]} dialog={false} />
		</div>
	)
}
