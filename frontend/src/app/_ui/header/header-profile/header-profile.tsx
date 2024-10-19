import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from "@/components/components/popover";
import { Button } from "@/components/components/button";
import { cookies } from "next/headers";
import Image from "next/image";
import { Suspense, useCallback } from "react";
import { LogOut, Settings, SunMoon } from "lucide-react";
import Link from "next/link";
import { HeaderProfileTheme } from "./header-profile-theme";

export function HeaderProfile() {
	return (
		<Suspense fallback={null}>
			<_HeaderProfile />
		</Suspense>
	);
}

function _HeaderProfile() {
	const cookiez = cookies();

	cookiez.toString();

	const loggedIn = true;

	if (!loggedIn) {
		return (
			<div>
				<Button variant="outline">Log In</Button>
			</div>
		);
	}

	const user = {
		username: "fitstagram",
		avatar: "https://avatars.githubusercontent.com/u/7655549?v=4",
	};

	return (
		<div>
			<Popover>
				<PopoverTrigger>
					<Button asChild variant="outline" className="space-x-2">
						<span>{user.username}</span>
						<Image
							unoptimized={true}
							src={user.avatar}
							alt={user.username}
							width={24}
							height={24}
						/>
					</Button>
				</PopoverTrigger>
				<PopoverContent
					sideOffset={12}
					className="animate-[popover_150ms_ease-out] w-[200px]"
				>
					<div className="flex flex-col items-center p-2 space-y-5">
						<div className="space-y-2 flex flex-col items-center">
							<Image
								unoptimized={true}
								src={user.avatar}
								alt={user.username}
								width={96}
								height={96}
								className="rounded-full"
							/>
							<span>
								<Link href="/profile">{user.username}</Link>
							</span>
						</div>
						<div className="flex flex-row justify-between space-x-3">
							<HeaderProfileTheme />
							<div className="rounded-full border p-2 border-accent hover:border-accent-foreground">
								<Link href="/settings">
									<Settings width={20} height={20} />
								</Link>
							</div>
							<div className="rounded-full border p-2 border-accent hover:border-accent-foreground">
								<LogOut width={20} height={20} />
							</div>
						</div>
					</div>
				</PopoverContent>
			</Popover>
		</div>
	);
}
