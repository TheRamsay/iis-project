import type React from "react";
import {
	Dialog,
	DialogContent,
	DialogTrigger,
} from "@/components/components/dialog";
import Image from "next/image";
import Link from "next/link";

interface PostDialog {
	children: React.ReactNode;
	post: {
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
	};
}

export function PostDialog({ children, post }: PostDialog) {
	return (
		<Dialog>
			<DialogTrigger>{children}</DialogTrigger>
			<DialogContent className="!p-0 !w-[90vh] !max-w-[90vh] h-full max-h-[70vh]">
				<div className="flex flex-row w-full h-full">
					<div className="relative border-accent border-r h-full">
						<Image
							unoptimized
							src={post.image.src}
							alt="image"
							width={post.image.width}
							height={post.image.height}
							className="h-full w-full object-contain"
						/>
					</div>
					<div className="flex flex-col justify-between flex-grow">
						<div className="">
							<div className="p-4 border-b border-accent flex flex-row space-x-3 items-center">
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
							<div className="p-4">komentare</div>
						</div>
						<div>
							<div className="border-y border-accent p-4">nastroje</div>
							<div className="p-4">pridat komentar</div>
						</div>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
