'use client'

import { SunMoon } from 'lucide-react'
import { useTheme } from 'next-themes'

export function HeaderProfileTheme() {
	const { resolvedTheme, setTheme } = useTheme()

	return (
		<div
			className="rounded-full border p-2 border-accent hover:border-accent-foreground cursor-pointer"
			onClick={() => {
				setTheme(resolvedTheme === 'dark' ? 'light' : 'dark')
			}}
		>
			<SunMoon width={20} height={20} />
		</div>
	)
}
