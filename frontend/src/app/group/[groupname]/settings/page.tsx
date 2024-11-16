import { getSession } from '@/app/_lib/auth/get-session'
import { redirect } from 'next/navigation'

export default async function Page({
	params,
}: { params: { groupname: string } }) {
	const session = await getSession()

	if (!session) {
		return redirect('/login')
	}

	const isManager = await true

	if (!isManager) {
		return <div>You are not a manager of this group.</div>
	}

	return <>Group Settings</>
}
