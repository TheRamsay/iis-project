import { fetchGroupByUsername } from '../../../_lib/fetch-groups-by-username'
import { GroupDelete } from './_ui/group-delete'

export default async function Page({
	params,
}: { params: { groupname: string } }) {
	const group = await fetchGroupByUsername(params.groupname)

	return <GroupDelete group={group} />
}
