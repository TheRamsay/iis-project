"use client";

import classNames from "classnames";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useMemo } from "react";

const sidebarItems = [
	{ name: "Dashboard", path: "/admin" },
	{ name: "Users", path: "/admin/users" },
];

export function Sidebar() {
	const pathname = usePathname();

	const active_item = useMemo(
		() => sidebarItems.find((item) => item.path === pathname),
		[pathname],
	);

	return (
		<div className="space-y-2 font-semibold text-lg">
			{sidebarItems.map((item) => (
				<div
					key={item.path}
					className={classNames(
						active_item?.path !== item.path && "text-opacity-60 text-white",
					)}
				>
					<Link href={item.path}>{item.name}</Link>
				</div>
			))}
		</div>
	);
}
