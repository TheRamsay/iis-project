'use client'

import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'
import type { schema } from '@/app/_lib/db'
import { ErrorTooltip } from '@/app/_ui/error-tooltip'
import { Pagination } from '@/app/_ui/pagination'
import { DataTable, Loader, TextField } from '@/components/components'
import { useMutation, useQuery } from '@tanstack/react-query'
import type { ColumnDef } from '@tanstack/react-table'
import classNames from 'classnames'
import { CheckIcon, SearchIcon, XIcon } from 'lucide-react'
import { useCallback, useMemo, useState } from 'react'
import { fetchGroupByUsername } from '../../../_lib/fetch-groups-by-username'

type Entry = Pick<typeof schema.user.$inferSelect, 'id' | 'username'>

const columns = [
	{
		accessorFn: (row) => row.username,
		header: 'Username',
		enableSorting: false,
	},
	{
		id: 'manage',
		cell: ({ row }) => {
			const [handled, setHandled] = useState<boolean>(false)

			const { mutate, error, isPending } = useMutation({
				mutationKey: ['group-handle-user-request', row.original.id],
				mutationFn: async (accept: boolean) => {
					const response = await backendFetch(
						`/api/group-join-requests/${row.original.id}/${accept ? 'approve' : 'reject'}`,
						{
							method: 'POST',
						},
					)

					await checkResponse(response)
				},
				onSuccess: () => setHandled(true),
			})

			return (
				<div className="justify-end flex w-full">
					{!handled && (
						<div className="flex space-x-2 items-center">
							{isPending && <Loader />}
							<ErrorTooltip error={error} size="small" />
							<div
								onClick={() => mutate(true)}
								className={classNames(
									'p-1 bg-secondary rounded-md cursor-pointer text-green-500',
									isPending && 'cursor-not-allowed',
								)}
							>
								<CheckIcon width={16} height={16} />
							</div>
							<div
								className={classNames(
									'p-1 bg-secondary rounded-md cursor-pointer text-red-500',
									isPending && 'cursor-not-allowed',
								)}
							>
								<XIcon onClick={() => mutate(false)} width={16} height={16} />
							</div>
						</div>
					)}
				</div>
			)
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

	const { data: groupId } = useQuery({
		queryKey: ['group-id', groupname],
		queryFn: async () => {
			const group = await fetchGroupByUsername(groupname)

			return group.id
		},
	})

	const { data, isLoading } = useQuery({
		queryKey: ['group-requests', groupId, filters],
		queryFn: async () => {
			const response = await backendFetch(`/api/groups/${groupId}/requests`)

			await checkResponse(response)

			// biome-ignore lint/suspicious/noExplicitAny: <explanation>
			const data: any[] = await response.json()

			const mapped = data
				.filter((entry) => entry.status === 'pending')
				.map((entry) => ({
					id: entry.id,
					username: entry.user.username,
				}))

			const filtered = mapped.filter((entry) => {
				if (filters.search) {
					return entry.username.includes(filters.search)
				}
				return true
			})

			return filtered
		},
		enabled: !!groupId,
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
					<h1 className="text-3xl font-medium">Requests</h1>
					<FilterSearch
						value={filters.search || ''}
						setValue={(value) => {
							setFilters({ ...filters, search: value })
						}}
					/>
				</div>
				<DataTable columns={columns} data={currentData} loading={isLoading} />
			</div>

			<Pagination
				page={pageIndex}
				canGoPrevious={canGoPrevious}
				canGoNext={canGoNext}
				onPrevious={onPrevious}
				onNext={onNext}
			/>
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
