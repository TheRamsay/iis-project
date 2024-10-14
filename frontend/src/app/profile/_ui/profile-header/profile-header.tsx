import Image from "next/image";

interface ProfileHeader {
	profile_id: string;
}

export function ProfileHeader({ profile_id }: ProfileHeader) {
	const profile = {
		username: "johndoe",
		bio: "I am a person.",
		avatar: {
			src: "https://www.gravatar.com/avatar/",
			width: 32,
			height: 32,
		},
	};

	return (
		<div className="flex-col flex">
			<div className="flex-row flex items-center space-x-6">
				<div className="relative h-28 w-28 rounded-full">
					<Image
						unoptimized
						src={profile.avatar.src}
						fill
						alt={profile.username}
						className="rounded-full"
					/>
				</div>
				<div>
					<span className="text-2xl">{profile.username}</span>
				</div>
			</div>
		</div>
	);
}
