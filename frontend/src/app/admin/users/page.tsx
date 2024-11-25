'use client'

import { Button } from '@/components/components/button'
import { DataTable } from '@/components/components/data-table/data-table'
import { useInfiniteQuery, useQuery } from '@tanstack/react-query'
import type { ColumnDef } from '@tanstack/react-table'
import classNames from 'classnames'
import { ChevronLeftIcon, ChevronRightIcon, SearchIcon } from 'lucide-react'
import { useCallback, useEffect, useMemo, useState } from 'react'
import { UserModal } from './_ui/user-modal/user-modal'
import type { schema } from '@/app/_lib/db'
import { userType } from '../../../../drizzle/schema'
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '@/components/components/select'
import { TextField } from '@/components/components/text-field'
import { useSearchParams } from 'next/navigation'
import { fetchAllUsers } from '@/app/_lib/user/fetch-all-users'
import type { User } from '@/app/_lib/user/fetch-user'

const columns = [
	{
		accessorFn: (row) => row.username,
		header: 'Username',
		enableSorting: false,
	},
	{
		accessorFn: (row) => row.role,
		header: 'Role',
		enableSorting: false,
	},
	{
		accessorFn: (row) => row.isBlocked,
		header: 'Blocked',
		enableSorting: false,
		cell: ({ row }) => {
			if (row.original.isBlocked) {
				return <div className="text-red-500">Yes</div>
			}

			return <div className="text-green-500">No</div>
		},
	},
	{
		id: 'manage',
		cell: ({ row }) => {
			return (
				<div className="justify-end flex w-full">
					<UserModal username={row.original.username}>
						<Button variant="secondary">Manage</Button>
					</UserModal>
				</div>
			)
		},
		meta: {
			className: 'text-right',
		},
	},
] as ColumnDef<User, unknown>[]

type Filters = {
	username?: string
	isBlocked?: boolean
	role?: (typeof schema.user.$inferSelect)['userType']
}

const PAGE_SIZE = 10

export default function Page() {
	const [pageIndex, setPageIndex] = useState<number>(0)
	const [filters, setFilters] = useState<Filters>({})

	const searchParams = useSearchParams()

	const { data, isLoading } = useQuery({
		queryKey: ['admin-users', filters],
		queryFn: async () => {
			const users = await fetchAllUsers(filters)
			return users
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

	const usernameFromUrl = searchParams.get('username')

	return (
		<>
			<div className="h-full flex justify-between flex-col">
				<div className="space-y-4">
					<div className="flex w-full justify-between items-center">
						<h1 className="text-3xl font-medium">Users</h1>
						<FilterSearch
							value={filters.username || ''}
							setValue={(value) => {
								setFilters({ ...filters, username: value })
							}}
						/>
					</div>
					<div className="flex w-full justify-end space-x-8">
						<FilterBlocked
							value={filters.isBlocked}
							setValue={(value) => {
								setFilters({ ...filters, isBlocked: value })
							}}
						/>
						<FilterRole
							value={filters.role}
							setValue={(value) => {
								setFilters({ ...filters, role: value })
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
							className={classNames(
								canGoNext ? 'cursor-pointer' : 'opacity-50',
							)}
						>
							<ChevronRightIcon className="h-8 w-8" />
						</div>
					</div>
				</div>
			</div>
			{usernameFromUrl && <UserModal username={usernameFromUrl} open />}
		</>
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

function FilterBlocked({
	value,
	setValue,
}: {
	value: boolean | undefined
	setValue: (value: boolean | undefined) => void
}) {
	const [stringValue, setStringValue] = useState('')

	const stringValues = useMemo(() => {
		return ['All', 'Yes', 'No']
	}, [])

	useEffect(() => {
		if (value === undefined) {
			setStringValue('All')
		} else {
			setStringValue(value ? 'Yes' : 'No')
		}
	}, [value])

	return (
		<div className="flex flex-row space-x-2 items-center">
			<label htmlFor="blocked">Blocked</label>
			<Select
				value={stringValue}
				onValueChange={(stringValue) => {
					if (stringValue === 'All') {
						setValue(undefined)
					}

					if (stringValue === 'Yes') {
						setValue(true)
					}

					if (stringValue === 'No') {
						setValue(false)
					}
				}}
			>
				<SelectTrigger className="flex justify-between w-full min-w-[100px]">
					<SelectValue />
				</SelectTrigger>
				<SelectContent>
					{Object.values(stringValues).map((entry) => (
						<SelectItem key={entry} value={entry}>
							{entry}
						</SelectItem>
					))}
				</SelectContent>
			</Select>
		</div>
	)
}

function FilterRole({
	value,
	setValue,
}: {
	value: (typeof userType.enumValues)[number] | undefined
	setValue: (value: (typeof userType.enumValues)[number] | undefined) => void
}) {
	const values = useMemo(() => {
		return ['All', ...Object.values(userType.enumValues)]
	}, [])

	return (
		<div className="flex flex-row space-x-2 items-center">
			<label htmlFor="role">Role</label>
			<Select
				value={value || 'All'}
				onValueChange={(value) => {
					if (value === 'All') {
						return setValue(undefined)
					}

					setValue(value as (typeof userType.enumValues)[number])
				}}
			>
				<SelectTrigger className="flex justify-between w-full min-w-[170px]">
					<SelectValue />
				</SelectTrigger>
				<SelectContent>
					{Object.values(values).map((entry) => (
						<SelectItem key={entry} value={entry}>
							{entry}
						</SelectItem>
					))}
				</SelectContent>
			</Select>
		</div>
	)
}
