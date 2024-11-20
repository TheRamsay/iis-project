import { EditPostForm } from './_ui/edit-post-form'

export default function Page({
	params: { postId },
}: { params: { postId: string } }) {
	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Edit Post</h1>
			<EditPostForm postId={postId} />
		</div>
	)
}
