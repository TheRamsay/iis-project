import {
	useCallback,
	useEffect,
	useState,
	type FC,
	type ReactNode,
} from 'react'
import {
	animated,
	useSpring,
	type SpringValue,
	type OnRest,
	type Controller,
} from 'react-spring'
import useResizeObserver from 'use-resize-observer'

interface Collapsible {
	open: boolean
	children: ReactNode
	className?: string
	afterChange?: () => void
}

export const Collapsible: FC<Collapsible> = ({
	className,
	open,
	children,
	afterChange,
}) => {
	const [finishedOpening, setFinishedOpening] = useState(open)
	const { ref, height } = useResizeObserver()

	const onRest = useCallback(
		(props: { value: { height: number } }) => {
			if (props.value.height) {
				setFinishedOpening(true)
			}

			afterChange?.()
		},
		[afterChange],
	)

	useEffect(() => {
		if (!open) {
			setFinishedOpening(false)
		}
	})

	const props = useSpring({
		height: open ? (height ?? 0) : 0,

		config: {
			mass: 1.2,
			tension: 300,
			friction: 20,
			clamp: true,
			velocity: 0.01,
		},
		onRest,
	})

	return (
		<animated.div
			style={{
				...props,
				width: '100%',
				willChange: 'height',
				overflow: finishedOpening ? 'visible' : 'hidden',
			}}
		>
			<div ref={ref} className={className}>
				{children}
			</div>
		</animated.div>
	)
}
