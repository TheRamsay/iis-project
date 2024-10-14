import { Heart, MessageCircle } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { PostLikeButton } from "./post-like-button";
import { PostCommentButton } from "./post-comment-button";
import { PostDialog } from "@/app/_ui/post-dialog/post-dialog";
import { PostComments } from "@/app/_ui/post-comments/post-comments";

interface Post {
	id: number;
	image: {
		src: string;
		width: number;
		height: number;
	};
	caption: string;
	user: {
		username: string;
		avatar: string;
	};
	like_count: number;
	comments: {
		id: number;
		user: {
			username: string;
			avatar: string;
		};
		content: string;
	}[];
}

export function Post(post: Post) {
	return (
		<div key={post.id} className="w-full flex flex-col space-y-3">
			<div>
				<div className="flex flex-row space-x-3 p-2 items-center">
					<Image
						unoptimized={true}
						src={post.user.avatar}
						alt="avatar"
						width={32}
						height={32}
						className="rounded-full"
					/>
					<Link href={`/profile/${post.user.username}`}>
						<span>{post.user.username}</span>
					</Link>
				</div>
				<PostDialog post={post}>
					<div className="relative h-full w-full">
						<Image
							unoptimized={true}
							src={post.image}
							alt="image"
							width={post.image.width}
							height={post.image.height}
							className="h-auto w-full object-contain"
						/>
					</div>
				</PostDialog>
			</div>
			<div className="space-x-5 flex">
				<PostLikeButton post_id={post.id} like_count={post.like_count} />
				<PostCommentButton
					post_id={post.id}
					comment_count={post.comments.length}
				/>
			</div>
			<PostComments post={post} />
		</div>
	);
}
