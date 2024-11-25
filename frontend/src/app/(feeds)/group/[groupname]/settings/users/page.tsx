'use client'

import type { schema } from '@/app/_lib/db'
import { ErrorTooltip } from '@/app/_ui/error-tooltip'
import { Button, DataTable, Loader, TextField } from '@/components/components'
import { useInfiniteQuery, useMutation, useQuery } from '@tanstack/react-query'
import type { ColumnDef } from '@tanstack/react-table'
import classNames from 'classnames'
import { ChevronLeftIcon, ChevronRightIcon, SearchIcon } from 'lucide-react'
import { useCallback, useMemo, useState } from 'react'
import { fetchGroupByUsername } from '../../../_lib/fetch-groups-by-username'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'

type Entry = Pick<typeof schema.user.$inferSelect, 'id' | 'username'> & {
	role: 'manager' | 'member'
}

const columns = [
	{
		accessorFn: (row) => row.username,
		header: 'Username',
		enableSorting: false,
	},
	{
		header: 'Role',
		cell: ({ row }) => {
			const role = row.original.role

			if (role === 'manager') {
				return <div>Manager</div>
			}

			return <div>Member</div>
		},
	},
	{
		id: 'manage',
		cell: ({ row }) => {
			const role = row.original.role

			const [kicked, setKicked] = useState<boolean>(false)

			const { mutate, error, isPending } = useMutation({
				mutationKey: ['group-kick-user', row.original.id],
				mutationFn: async () => {
					await new Promise((resolve) => setTimeout(resolve, 1000))
					throw new Error('Failed to kick user')
				},
				onSuccess: () => setKicked(true),
			})

			if (role === 'member') {
				return (
					<div className="w-full flex justify-end">
						{!kicked && (
							<div className="flex items-center space-x-2">
								{isPending && <Loader />}
								<ErrorTooltip error={error} size="small" />
								<Button variant="outline" onClick={() => mutate()}>
									Kick
								</Button>
							</div>
						)}
					</div>
				)
			}

			return null
		},
		meta: {
			className: 'text-right',
		},
	},
] as ColumnDef<Entry, unknown>[]

const PAGE_SIZE = 10

type Filters = {
	search?: string
}

export default function Page({
	params: { groupname },
}: { params: { groupname: string } }) {
	const [pageIndex, setPageIndex] = useState<number>(0)
	const [filters, setFilters] = useState<Filters>({})

	const { data: group } = useQuery({
		queryKey: ['group-id', groupname],
		queryFn: async () => {
			const group = await fetchGroupByUsername(groupname)

			return group
		},
	})

	const { data, isLoading } = useQuery({
		queryKey: ['group-users', group],
		queryFn: async () => {
			const response = await backendFetch(`/api/groups/${group.id}/members`)

			await checkResponse(response)

			// biome-ignore lint/suspicious/noExplicitAny: <explanation>
			const data: any[] = await response.json()

			return data.map((entry) => ({
				id: entry.id,
				username: entry.username,
				role: entry.id === group?.admin.id ? 'manager' : 'member',
			}))
		},
	})

	const currentData = useMemo(() => {
		return data?.slice(pageIndex * PAGE_SIZE, (pageIndex + 1) * PAGE_SIZE) || []
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
