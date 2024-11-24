import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from '@/components/components/popover'
import { Button } from '@/components/components/button'
import { Suspense } from 'react'
import { LogOut, Settings } from 'lucide-react'
import Link from 'next/link'
import { HeaderProfileTheme } from './header-profile-theme'
import { getSession } from '@/app/_lib/auth/get-session'
import { Avatar } from '../../avatar'

export function HeaderProfile() {
	return (
		<Suspense fallback={null}>
			<_HeaderProfile />
		</Suspense>
	)
}

async function _HeaderProfile() {
	const session = await getSession()

	if (!session) {
		return (
			<div className="space-x-2">
				<Link href="/login">
					<Button variant="outline">Log In</Button>
				</Link>
				<Link href="/register">
					<Button variant="outline">Register</Button>
				</Link>
			</div>
		)
	}

	return (
		<div>
			<Popover>
				<PopoverTrigger>
					<Button asChild variant="outline" className="space-x-2">
						<span>{session.username}</span>
						<Avatar
							unoptimized={true}
							src={session.avatar.src}
							alt="avatar"
							size={24}
							name={session.username}
						/>
					</Button>
				</PopoverTrigger>
				<PopoverContent
					sideOffset={12}
					className="animate-[popover_150ms_ease-out] w-[200px]"
				>
					<div className="flex flex-col items-center p-2 space-y-5">
						<div className="space-y-2 flex flex-col items-center">
							<Avatar
								unoptimized={true}
								src={session.avatar.src}
								alt={session.username}
								size={96}
								name={session.username}
								className="rounded-full"
							/>
							<span>
								<Link href="/profile">{session.username}</Link>
							</span>
						</div>
						<div className="flex flex-row justify-between space-x-3">
							<HeaderProfileTheme />
							<Link href="/settings">
								<div className="rounded-full border p-2 border-accent hover:border-accent-foreground">
									<Settings width={20} height={20} />
								</div>
							</Link>
							<a href="/logout">
								<div className="rounded-full border p-2 border-accent hover:border-accent-foreground">
									<LogOut width={20} height={20} />
								</div>
							</a>
						</div>
					</div>
				</PopoverContent>
			</Popover>
		</div>
	)
}
