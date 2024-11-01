import { ProfilePage } from '../_ui/profile-page'

export default function Page({ params }: { params: { username: string } }) {
	return <ProfilePage username={params.username} />
}
