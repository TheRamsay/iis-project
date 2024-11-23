import { GroupSettingsForm } from './_ui/group-settings-form'

export default async function Page({
	params,
}: { params: { groupname: string } }) {
	// TODO: endpoint
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
