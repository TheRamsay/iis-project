import { dummyPosts } from '@/app/_types/post'
import { PostDialogContent } from '@/app/_ui/post/post-dialog/post-dialog-content'
import { fetchPost } from '../../_lib/fetch-post'

export default async function Page({
	params: { postId },
}: { params: { postId: string } }) {
	const post = await fetchPost(postId)

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Post</h1>
			<PostDialogContent post={post} dialog={false} />
		</div>
	)
}
