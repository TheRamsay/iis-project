import { getSession } from '@/app/_lib/auth/get-session'
import { redirect } from 'next/navigation'

export default async function Page() {
	const session = await getSession()

	if (!session) {
		return redirect('/login')
	}

	// TODO: endpoint to create a group

	return <>create</>
}
