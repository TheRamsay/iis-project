import { getSession } from '@/app/_lib/auth/get-session'
import { redirect } from 'next/navigation'
import { FormCreateGroup } from './_ui/form-create-group'

export default async function Page() {
	const session = await getSession()

	if (!session) {
		return redirect('/')
	}

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Create Group</h1>
			<FormCreateGroup />
		</div>
	)
}
