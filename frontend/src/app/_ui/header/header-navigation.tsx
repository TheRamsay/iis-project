import { getSession } from '@/app/_lib/auth/get-session'
import { isMinModerator } from '@/app/_lib/get-permission-level'
import { HomeIcon, Settings, SquarePlusIcon, UsersIcon } from 'lucide-react'
import Link from 'next/link'

export async function HeaderNavigation() {
	const session = await getSession()

	return (
		<div className="flex flex-row gap-4">
			<Link href="/">
				<HomeIcon width={24} height={24} />
			</Link>
			{session && (
				<>
					<Link href="/post/submit">
						<SquarePlusIcon width={24} height={24} />
					</Link>
					<Link href="/group">
						<UsersIcon width={24} height={24} />
					</Link>
				</>
			)}
			{isMinModerator(session?.role) && (
				<Link href="/admin">
					<Settings width={24} height={24} />
				</Link>
			)}
		</div>
	)
}
