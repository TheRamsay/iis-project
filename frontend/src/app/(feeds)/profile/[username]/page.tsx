import { ProfilePage } from '../_ui/profile-page'

export default function Page({
	params,
	searchParams,
}: { params: { username: string }; searchParams: Record<string, string> }) {
	return <ProfilePage username={params.username} searchParams={searchParams} />
}
