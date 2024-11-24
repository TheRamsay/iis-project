import { redirect } from 'next/navigation'
import { ProfilePage } from './_ui/profile-page'
import { getSession } from '../../_lib/auth/get-session'

export default async function Page({
	searchParams,
}: { searchParams: Record<string, string> }) {
	const session = await getSession()

	if (!session) {
		return redirect('/login')
	}

	return <ProfilePage username={session.username} searchParams={searchParams} />
}
