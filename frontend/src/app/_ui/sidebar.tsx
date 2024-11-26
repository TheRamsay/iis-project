'use client'

import classNames from 'classnames'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useMemo } from 'react'

export type SidebarItem = {
	name: string
	path: string
	className?: string
}

interface Sidebar {
	items: SidebarItem[]
}

export function Sidebar({ items }: Sidebar) {
	const pathname = usePathname()

	const activeItem = useMemo(
		() => items.find((item) => item.path === pathname),
		[items, pathname],
	)

	return (
		<div className="space-y-2 font-semibold text-lg">
			{items.map((item) => (
				<div
					key={item.path}
					className={classNames(
						activeItem?.path !== item.path &&
							'!text-opacity-70 text-white hover:!text-opacity-95',
						item.className,
						'text-opacity-0',
					)}
				>
					<Link href={item.path} className="" prefetch={false}>
						{item.name}
					</Link>
				</div>
			))}
		</div>
	)
}
