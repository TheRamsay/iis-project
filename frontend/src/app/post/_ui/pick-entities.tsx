'use client'

import {
	SimpleSearch,
	type SimpleSearchDataProps,
} from '@/app/_ui/simple-search'
import { UserAvatarName } from '@/app/_ui/user/user-avatar-name'
import { SkeletonText } from '@/components/components'
import { useQuery } from '@tanstack/react-query'
import { CheckIcon, MinusCircleIcon } from 'lucide-react'
import { useCallback, useState } from 'react'

export type Entity = {
	id: string
	username: string
	avatar: {
		src: string
		width: number
		height: number
	}
}

interface PickEntities {
	type: 'user' | 'group'
	list: Entity[]
	onChange: (newList: Entity[]) => void
}

export function PickEntities({ type, list, onChange }: PickEntities) {
	const [query, setQuery] = useState('')

	const { data, isLoading, isError } = useQuery<Entity[]>({
		queryKey: ['query-followed', type, query],
		queryFn: async () => {
			return [
				{
					avatar: {
						src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
						width: 128,
						height: 128,
					},
					id: '1',
					username: 'fitstagram',
				},
			]
		},
	})

	const onDelete = useCallback(
		(entity: Entity) => {
			onChange(list.filter((e) => e.id !== entity.id))
		},
		[list, onChange],
	)

	const onClick = useCallback(
		(entity: Entity) => {
			if (list.find((e) => e.id === entity.id)) {
				onDelete(entity)
			} else {
				onChange([...list, entity])
			}
		},
		[list, onChange, onDelete],
	)

	const dataRenderer = useCallback(
		(params: SimpleSearchDataProps<Entity[]>) => {
			return <Results {...params} onClick={onClick} selectedData={list} />
		},
		[list, onClick],
	)

	return (
		<div className="space-y-2">
			<div className="flex items-center space-x-2">
				<div className="capitalize">{type}s</div>
				<SimpleSearch
					query={query}
					setQuery={setQuery}
					placeholder={`Search ${type}s`}
					data={data}
					isError={isError}
					isLoading={isLoading}
					dataRenderer={dataRenderer}
				/>
			</div>
			<div>
				{list.map((entity) => (
					<div
						key={entity.id}
						className="flex w-full justify-between items-center"
					>
						<UserAvatarName user={entity} />
						<MinusCircleIcon
							color="red"
							width={20}
							height={20}
							className="cursor-pointer"
							onClick={() => onDelete(entity)}
						/>
					</div>
				))}
			</div>
		</div>
	)
}

interface Results {
	isLoading: boolean
	isError: boolean
	data: Entity[] | undefined
	selectedData: Entity[]
	onClick: (entity: Entity) => void
}

function Results({
	data,
	isError,
	isLoading,
	selectedData,
	onClick: _onClick,
}: Results) {
	if (isError) {
		return (
			<div className="px-4 pt-4 pb-2 gap-2 flex justify-center w-full text-sm">
				An unexpected error has occured.
			</div>
		)
	}

	if (isLoading) {
		return (
			<div className="px-4 py-2 gap-2">
				<SkeletonText fontSize="sm" />
				<SkeletonText fontSize="sm" />
				<SkeletonText fontSize="sm" />
			</div>
		)
	}

	const onClick = useCallback(
		(event: React.MouseEvent, entity: Entity) => {
			event.preventDefault()
			event.stopPropagation()

			_onClick(entity)
		},
		[_onClick],
	)

	return (
		<div className="px-4 py-2 gap-2 text-sm">
			<div className="text-sm">
				{data?.map((entity) => (
					<div
						key={entity.id}
						onClick={(event) => onClick(event, entity)}
						className="w-full justify-between flex cursor-pointer"
					>
						<UserAvatarName user={entity} size="small" disableLink />
						{selectedData.find((d) => d.id === entity.id) ? (
							<CheckIcon color="green" width={20} height={20} />
						) : null}
					</div>
				))}
			</div>
		</div>
	)
}
