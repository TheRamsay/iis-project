import { cookies } from "next/headers";
import { Feed } from "./_ui/feed/feed";

export default function Home() {
	const cookiez = cookies();

	cookiez.toString();

	const entry = {
		id: 1,
		image: {
			src: "https://avatars.githubusercontent.com/u/7655549?v=4",
			width: 128,
			height: 128,
		},
		caption: "This is a post",
		user: {
			username: "fitstagram",
			avatar: "https://avatars.githubusercontent.com/u/7655549?v=4",
		},
		like_count: 0,
		comments: [
			{
				id: 1,
				user: {
					username: "fitstagram",
					avatar: "https://avatars.githubusercontent.com/u/7655549?v=4",
				},
				content: "This is a comment",
			},
			{
				id: 2,
				user: {
					username: "fitstagram",
					avatar: "https://avatars.githubusercontent.com/u/7655549?v=4",
				},
				content: "This is a comment",
			},
		],
	};

	const feed = [entry, { ...entry, id: 2 }];

	return (
		<div className="">
			<Feed data={feed} />
		</div>
	);
}
