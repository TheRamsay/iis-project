import {
	HoverCard,
	HoverCardContent,
	HoverCardTrigger,
} from '@/components/components/hover-card'
import { CircleAlert } from 'lucide-react'

interface ErrorTooltip {
	error: Error | null
	size?: 'small' | 'full'
}

export function ErrorTooltip({ error, size }: ErrorTooltip) {
	if (!error) {
		return null
	}

	const pix = size === 'small' ? 16 : 24

	return (
		<HoverCard>
			<HoverCardTrigger>
				<CircleAlert className="text-red-500" width={pix} height={pix} />
			</HoverCardTrigger>
			<HoverCardContent>{error.message}</HoverCardContent>
		</HoverCard>
	)
}
