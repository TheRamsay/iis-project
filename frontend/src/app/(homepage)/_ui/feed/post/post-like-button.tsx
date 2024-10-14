"use client";

import { Heart } from "lucide-react";
import { useCallback } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import classNames from "classnames";

interface PostLikeButton {
	post_id: number;
	like_count: number;
}

type LikeData = {
	current_likes: number;
	is_liked: boolean;
};

export function PostLikeButton({ post_id, like_count }: PostLikeButton) {
	const is_logged_in = false;
	const user_id = 1;

	const queryClient = useQueryClient();

	const { data, refetch } = useQuery<LikeData>({
		queryKey: ["like", user_id, post_id],
		queryFn: async () => {
			return {
				current_likes: 0,
				is_liked: false,
			};
		},
		enabled: is_logged_in,
		placeholderData: {
			current_likes: like_count,
			is_liked: false,
		},
	});

	const { mutateAsync } = useMutation({
		mutationKey: ["like", post_id],
		mutationFn: async () => {},
		onMutate: async () => {
			await queryClient.cancelQueries({ queryKey: ["like", user_id, post_id] });
			const previous = queryClient.getQueryData<LikeData>([
				"like",
				user_id,
				post_id,
			]);
			queryClient.setQueryData<LikeData>(["like", user_id, post_id], (old) => {
				if (old) {
					return {
						current_likes: old.current_likes + 1,
						is_liked: !old.is_liked,
					};
				}
				return {
					current_likes: like_count + 1,
					is_liked: true,
				};
			});

			return { previous };
		},
		onSettled: () => {
			refetch();
		},
	});

	return (
		<div className="space-x-3 flex items-center">
			<Heart
				className={classNames(
					data?.is_liked && "fill-red-600 text-red-600",
					is_logged_in && "cursor-pointer",
				)}
				width={28}
				height={28}
				onClick={() => mutateAsync()}
			/>
			<span className="font-semibold">{data?.current_likes}</span>
		</div>
	);
}
