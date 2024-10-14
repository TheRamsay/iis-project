import { Suspense } from "react";
import { ProfileHeader } from "./profile-header/profile-header";
import { ProfileFeed } from "./profile-feed";
import { Separator } from "@/components/components/separator";

interface ProfilePage {
	profile_id: string;
}

export function ProfilePage({ profile_id }: ProfilePage) {
	return (
		<Suspense fallback={null}>
			<_ProfilePage profile_id={profile_id} />
		</Suspense>
	);
}

function _ProfilePage({ profile_id }: ProfilePage) {
	return (
		<div className="space-y-4">
			<ProfileHeader profile_id={profile_id} />
			<Separator orientation="horizontal" className="!bg-accent" />
			<ProfileFeed profile_id={profile_id} />
		</div>
	);
}
