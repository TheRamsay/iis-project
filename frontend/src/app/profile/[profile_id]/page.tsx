import { ProfilePage } from "../_ui/profile-page";

export default function Page({ params }: { params: { profile_id: string } }) {
	return <ProfilePage profile_id={params.profile_id} />;
}
