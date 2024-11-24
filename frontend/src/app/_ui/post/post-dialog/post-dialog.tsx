import type React from 'react'
import {
	Dialog,
	DialogContent,
	DialogTrigger,
} from '@/components/components/dialog'
import { PostDialogContent } from './post-dialog-content'
import type { Post } from '@/app/_types/post'

export interface PostDialog {
	children: React.ReactNode
	post: Post
	groupModeratorIdList?: string[]
}

export function PostDialog({
	children,
	post,
	groupModeratorIdList,
}: PostDialog) {
	return (
		<Dialog>
			<DialogTrigger className="w-full">{children}</DialogTrigger>
			<DialogContent className="!p-0 !w-[90vw] !max-w-[90vw] h-full max-h-[90vh]">
				<PostDialogContent
					post={post}
					groupModeratorIdList={groupModeratorIdList}
				/>
			</DialogContent>
		</Dialog>
	)
}
