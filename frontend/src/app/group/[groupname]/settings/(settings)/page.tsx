import { getSession } from '@/app/_lib/auth/get-session'
import { redirect } from 'next/navigation'
import { GroupSettingsForm } from './_ui/group-settings-form'

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

	const group = {
		id: '1',
		name: 'Group 1',
	}

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Group Settings</h1>
			<GroupSettingsForm groupId={group.id} />
		</div>
	)
}
