import { UserForm } from './_ui/user-form'

export default function Page() {
	const userId = 'uu-uuui-ii'

	return (
		<div className="w-full space-y-8">
			<h1 className="text-3xl font-medium">Settings</h1>
			<UserForm userId={userId} />
		</div>
	)
}
