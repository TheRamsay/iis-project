'use client'

import type { schema } from '@/app/_lib/db'
import { Button, DataTable, Loader, TextField } from '@/components/components'
import { useInfiniteQuery, useMutation } from '@tanstack/react-query'
import type { ColumnDef } from '@tanstack/react-table'
import classNames from 'classnames'
import { ChevronLeftIcon, ChevronRightIcon, SearchIcon } from 'lucide-react'
import { useCallback, useMemo, useState } from 'react'

type Entry = Pick<typeof schema.user.$inferSelect, 'id' | 'username'>

const columns = [
	{
		accessorFn: (row) => row.username,
		header: 'Username',
		enableSorting: false,
	},
	{
		header: 'Role',
		cell: ({ row }) => {
			const role: 'manager' | 'member' = 'manager'

			if (role === 'manager') {
				return <div>Manager</div>
			}

			return <div>Member</div>
		},
	},
	{
		id: 'manage',
		cell: ({ row }) => {
			const role = 'member'

			const [kicked, setKicked] = useState<boolean>(false)

			const { mutate, isPending } = useMutation({
				mutationKey: ['group-kick-user', row.original.id],
				mutationFn: async () => {
					await new Promise((resolve) => setTimeout(resolve, 1000))
				},
				onSuccess: () => setKicked(true),
			})

			if (role === 'member') {
				return (
					<div className="w-full flex justify-end">
						{!kicked && (
							<div className="flex items-center space-x-2">
								{isPending && <Loader />}
								<Button variant="outline" onClick={() => mutate()}>
									Kick
								</Button>
							</div>
						)}
					</div>
				)
			}

			return <div className="justify-end flex w-full">?</div>
		},
		meta: {
			className: 'text-right',
		},
	},
] as ColumnDef<Entry, unknown>[]

const Adata: Entry[] = [
	{
		id: '1',
		username: 'fitstagram',
	},
	{
		id: '2',
		username: 'remzak.pepak',
	},
	{
		id: '3',
		username: 'padi142',
	},
	{
		id: '4',
		username: 'verka',
	},
	{
		id: '5',
		username: 'oliverova.knizka',
	},
]

const PAGE_SIZE = 10

type Filters = {
	search?: string
}

export default function Page({
	params: { groupname },
}: { params: { groupname: string } }) {
	const [pageIndex, setPageIndex] = useState<number>(0)
	const [filters, setFilters] = useState<Filters>({})

	const { data, isLoading } = useInfiniteQuery({
		queryKey: ['group-users', groupname],
		queryFn: ({ pageParam }) => {
			return Adata
		},
		initialPageParam: 0,
		getNextPageParam: (_1, _2, lastPageParam) => {
			// if(!hasNextPage) {
			//   return undefined
			// }

			return lastPageParam + 1
		},
	})

	const currentData = useMemo(() => {
		return data?.pages[pageIndex] || []
	}, [data, pageIndex])

	const [canGoPrevious, canGoNext] = useMemo(() => {
		return [pageIndex > 0, currentData.length === PAGE_SIZE]
	}, [pageIndex, currentData])

	const onPrevious = useCallback(() => {
		setPageIndex((oldPageIndex) => {
			if (canGoPrevious) {
				return oldPageIndex - 1
			}
			return oldPageIndex
		})
	}, [canGoPrevious])
	const onNext = useCallback(() => {
		setPageIndex((oldPageIndex) => {
			if (canGoNext) {
				return oldPageIndex + 1
			}
			return oldPageIndex
		})
	}, [canGoNext])

	return (
		<div className="h-full flex justify-between flex-col">
			<div className="space-y-4">
				<div className="flex w-full justify-between items-center">
					<h1 className="text-3xl font-medium">Users</h1>
					<FilterSearch
						value={filters.search || ''}
						setValue={(value) => {
							setFilters({ ...filters, search: value })
						}}
					/>
				</div>
				<DataTable columns={columns} data={currentData} loading={isLoading} />
			</div>

			<div className="w-full justify-between flex items-center">
				<div>Page: {pageIndex + 1}</div>
				<div className="flex space-x-2">
					<div
						onClick={onPrevious}
						className={classNames(
							canGoPrevious ? 'cursor-pointer' : 'opacity-50',
						)}
					>
						<ChevronLeftIcon className="h-8 w-8" />
					</div>
					<div
						onClick={onNext}
						className={classNames(canGoNext ? 'cursor-pointer' : 'opacity-50')}
					>
						<ChevronRightIcon className="h-8 w-8" />
					</div>
				</div>
			</div>
		</div>
	)
}

function FilterSearch({
	value,
	setValue,
}: {
	value: string
	setValue: (value: string) => void
}) {
	return (
		<div>
			<TextField
				type="text"
				value={value}
				onChange={(e) => setValue(e.target.value)}
				className="max-w-[300px]"
				placeholder="Search by username"
				icon={SearchIcon}
			/>
		</div>
	)
}
