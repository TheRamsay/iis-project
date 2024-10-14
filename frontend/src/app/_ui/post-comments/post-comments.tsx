import Image from "next/image";
import Link from "next/link";

interface PostComments {
	post: {
		id: number;
		comments: {
			id: number;
			user: {
				username: string;
				avatar: string;
			};
			content: string;
		}[];
	};
}

export function PostComments({ post }: PostComments) {
	return (
		<div className="space-y-2">
			{post.comments.splice(0, 2).map((comment) => (
				<div key={comment.id}>
					<div className="flex flex-row space-x-2 items-center">
						<Image
							unoptimized={true}
							src={comment.user.avatar}
							alt="avatar"
							width={20}
							height={20}
							className="rounded-full"
						/>
						<Link href={`/profile/${comment.user.username}`}>
							<span className="text-sm space-x-2">
								<span className="font-semibold">{comment.user.username}:</span>
								<span>{comment.content}</span>
							</span>
						</Link>
					</div>
				</div>
			))}
		</div>
	);
}
