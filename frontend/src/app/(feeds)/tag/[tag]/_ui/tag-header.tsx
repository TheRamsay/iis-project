import { TagHeaderActions } from './tag-header-actions'

interface TagHeader {
	tag: string
}

export async function TagHeader({ tag }: TagHeader) {
	return (
		<div className="flex w-full justify-between items-center space-x-4">
			<div className="flex-col flex">
				<div className="flex-row flex items-center space-x-6">
					<p className="space-y-2 [word-break:break-word]">
						<span className="text-2xl float-left">#{tag}</span>
					</p>
				</div>
			</div>
			<TagHeaderActions />
		</div>
	)
}
