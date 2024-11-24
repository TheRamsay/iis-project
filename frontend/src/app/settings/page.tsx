import { redirect } from 'next/navigation'
import { getSession } from '../_lib/auth/get-session'
import { UserForm } from './_ui/user-form'

export default async function Page() {
	const session = await getSession()

	if (!session) {
		redirect('/')
	}

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Settings</h1>
			<UserForm userId={session.userId} />
		</div>
	)
}
