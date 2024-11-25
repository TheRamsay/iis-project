import { fetchAllUsers } from '../_lib/user/fetch-all-users'

export default async function Page() {
	const users = await fetchAllUsers()

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Dashboard</h1>
			<div className="space-y-2">
				<div className="justify-between flex w-full">
					<span>User Count</span>
					<span>{users.length}</span>
				</div>
			</div>
		</div>
	)
}
