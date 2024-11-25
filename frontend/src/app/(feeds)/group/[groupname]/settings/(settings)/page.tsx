import { fetchGroupMembers } from '../../../_lib/fetch-group-members'
import { fetchGroupByUsername } from '../../../_lib/fetch-groups-by-username'

export default async function Page({
	params,
}: { params: { groupname: string } }) {
	const group = await fetchGroupByUsername(params.groupname)
	const members = await fetchGroupMembers(group.id)

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Dashboard</h1>
			<div className="space-y-2">
				<div className="justify-between flex w-full">
					<span>Name</span>
					<span>{group.groupname}</span>
				</div>
				<div className="justify-between flex w-full">
					<span>Admin</span>
					<span>{group.admin.username}</span>
				</div>
				<div className="justify-between flex w-full">
					<span>Member Count</span>
					<span>{members.length}</span>
				</div>
			</div>
		</div>
	)
}
