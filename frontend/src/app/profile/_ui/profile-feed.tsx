import { PostDialog } from '@/app/_ui/post-dialog/post-dialog'
import Image from 'next/image'

interface ProfileFeed {
	profileId: string
}

export function ProfileFeed({ profileId }: ProfileFeed) {
	const posts = Array(10)
		.fill(0)
		.map((_, i) => ({
			id: i,
			image: {
				src: 'https://picsum.photos/256',
				width: 800,
				height: 800,
			},
			caption: 'This is a post',
			user: {
				username: 'fitstagram',
				avatar: 'https://avatars.githubusercontent.com/u/7655549?v=4',
			},
			like_count: 0,
			comments: [
				{
					id: 1,
					user: {
						username: 'fitstagram',
						avatar: 'https://avatars.githubusercontent.com/u/7655549?v=4',
					},
					content: 'This is a comment',
				},
				{
					id: 2,
					user: {
						username: 'fitstagram',
						avatar: 'https://avatars.githubusercontent.com/u/7655549?v=4',
					},
					content: 'This is a comment',
				},
			],
		}))

	return (
		<div className="grid grid-cols-3 gap-3">
			{posts.map((post) => (
				<div key={post.id} className="w-full h-full relative aspect-square">
					<PostDialog post={post}>
						<Image
							src={post.image.src}
							fill
							unoptimized
							alt="image"
							className="object-contain"
						/>
					</PostDialog>
				</div>
			))}
		</div>
	)
}
