import { MiniatureFeed } from '@/app/(feeds)/_ui/feed/miniature-feed'
import { Separator } from '@/components/components/separator'
import { Suspense } from 'react'
import { TagHeader } from './_ui/tag-header'

export default function Page({
	params: { tag },
	searchParams,
}: { params: { tag: string }; searchParams: Record<string, string> }) {
	return (
		<div className="space-y-4">
			<TagHeader tag={tag} />
			<Separator orientation="horizontal" className="!bg-accent" />
			<Suspense>
				<Feed tag={tag} searchParams={searchParams} />
			</Suspense>
		</div>
	)
}

async function Feed({
	tag,
	searchParams,
}: { tag: string; searchParams: Record<string, string> }) {
	return <MiniatureFeed tag={tag} searchParams={searchParams} />
}
