import {
	HoverCard,
	HoverCardContent,
	HoverCardTrigger,
} from '@/components/components'
import { Info } from 'lucide-react'

export function InfoTooltip({ tooltip }: { tooltip: string }) {
	return (
		<HoverCard>
			<HoverCardTrigger>
				<Info width={16} height={16} className="mt-1" />
			</HoverCardTrigger>
			<HoverCardContent>{tooltip}</HoverCardContent>
		</HoverCard>
	)
}
