import { notFound, redirect } from 'next/navigation'
import { ProfilePage } from './_ui/profile-page'
import { cookies } from 'next/headers'

export default function Page() {
	const cookiez = cookies()

	const userId = 'my_user'

	if (!userId) {
		return redirect('/login')
	}

	return <ProfilePage profileId={userId} />
}
