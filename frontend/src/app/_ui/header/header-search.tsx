'use client'

import classNames from 'classnames'
import { SearchIcon, XIcon } from 'lucide-react'
import { useRef, useState } from 'react'
import { useOnClickOutside } from './use-on-click-outside'
import { SkeletonText } from '@/components/components/skeleton'
import { useQuery } from '@tanstack/react-query'
import { useDebounce } from 'use-debounce'
import { backendFetch, checkResponse } from '@/app/_lib/backend-fetch'
import { UserAvatarName } from '../user/user-avatar-name'
import Link from 'next/link'

type Data = {
	users: {
		id: string
		username: string
		avatar: { src: string }
	}[]
	groups: {
		id: string
		username: string
		avatar: { src: undefined }
	}[]
	tags: {
		name: string
	}[]
}

// Inspired by https://github.com/sushi-labs/sushiswap/blob/feature/egn-677/apps/web/src/app/(cms)/faq/(root)/components/search-box.tsx

export function HeaderSearch() {
	const [query, setQuery] = useState<string>('')
	const [open, setOpen] = useState<boolean>(false)

	const [debounced] = useDebounce(query, 300)

	const ref = useRef<HTMLDivElement>(null)

	useOnClickOutside(ref, () => setOpen(false))

	const { data, isLoading, isError } = useQuery({
		queryKey: ['header-search', debounced],
		queryFn: async () => {
			const response = await backendFetch(`/api/search?query=${debounced}`)

			await checkResponse(response)

			const data: {
				// biome-ignore lint/suspicious/noExplicitAny: <explanation>
				users: any[]
				// biome-ignore lint/suspicious/noExplicitAny: <explanation>
				groups: any[]
				// biome-ignore lint/suspicious/noExplicitAny: <explanation>
				tags: any[]
			} = await response.json()

			return {
				users: data.users
					// biome-ignore lint/suspicious/noExplicitAny: <explanation>
					.map((user: any) => ({
						id: user.id as string,
						username: user.username as string,
						avatar: { src: user.avatar_url as string },
					}))
					.slice(0, 5),
				groups: data.groups
					// biome-ignore lint/suspicious/noExplicitAny: <explanation>
					.map((group: any) => ({
						id: group.id as string,
						username: group.name as string,
						avatar: { src: undefined },
					}))
					.slice(0, 5),
				tags: data.tags
					// biome-ignore lint/suspicious/noExplicitAny: <explanation>
					.map((tag: any) => ({
						name: tag.tag as string,
					}))
					.sort((a, b) => a.name.length - b.name.length)
					.slice(0, 5),
			} satisfies Data
		},
		enabled: debounced.length > 0 && open,
	})

	return (
		<div className="flex flex-col gap-3 relative justify-center">
			<div className="z-10 flex w-full gap-4 min-w-[400px]">
				<div
					ref={ref}
					onFocus={() => setOpen(true)}
					className={classNames('rounded-md w-full border border-accent')}
				>
					<div className="flex items-center gap-2 pl-3 pr-3 h-10">
						<div className="flex gap-4 items-center w-full">
							<div className="w-6 h-6">
								<SearchIcon
									width={24}
									height={24}
									className="dark:text-gray-300 text-neutral-950"
								/>
							</div>
							<input
								value={query}
								onChange={(e) => setQuery(e.target.value)}
								placeholder="Search profiles, groups, tags..."
								className={classNames(
									'w-full dark:placeholder:accent-foreground',
									'p-0 bg-transparent border-none focus:outline-none focus:ring-0 w-full truncate font-medium text-left text-base md:text-sm placeholder:font-normal',
								)}
							/>
						</div>
						{query && (
							<XIcon
								onClick={() => setQuery('')}
								className="w-6 h-6 cursor-pointer dark:text-gray-500 text-neutral-950"
							/>
						)}
					</div>
					<div
						className={classNames(
							open
								? 'max-h-[335px] py-2 border-b border-l border-r -ml-[0.5px] scroll'
								: 'max-h-[0px]',
							'z-[100]  rounded-b-xl flex flex-col gap-2 overflow-hidden transition-all absolute w-full -mt-1',
							'bg-background border-accent',
						)}
					>
						<Results data={data} isError={isError} isLoading={isLoading} />
					</div>
				</div>
			</div>
		</div>
	)
}
interface Results {
	isLoading?: boolean
	isError: boolean
	data: Data | undefined
}

function Results({ data, isError, isLoading }: Results) {
	if (isLoading) {
		return (
			<div>
				<div className="mt-2 px-4">
					<SkeletonText />
				</div>
				<div className="px-4 py-2 gap-2">
					<SkeletonText fontSize="sm" />
					<SkeletonText fontSize="sm" />
					<SkeletonText fontSize="sm" />
				</div>
			</div>
		)
	}

	if (isError) {
		return (
			<div className="px-4 pt-4 pb-2 gap-2 flex justify-center w-full text-sm">
				An unexpected error has occured.
			</div>
		)
	}

	if (!data) {
		return <div />
	}

	return (
		<div>
			<div className="font-medium mt-2 px-4 text-gray-400">Users</div>
			<div className="px-4 py-2 gap-2 text-sm">
				{data.users.length ? (
					data.users.map((user) => <UserAvatarName key={user.id} user={user} />)
				) : (
					<div>No users found</div>
				)}
			</div>
			<div className="font-medium px-4 text-gray-400">Groups</div>
			<div className="px-4 py-2 gap-2 text-sm">
				{data.groups.length ? (
					data.groups.map((group) => (
						<UserAvatarName key={group.id} user={group} type="group" />
					))
				) : (
					<div>No groups found</div>
				)}
			</div>
			<div className="font-medium px-4 text-gray-400">Tags</div>
			<div className="px-4 py-2 gap-2">
				{data.tags.length ? (
					data.tags.map((tag) => (
						<div
							key={tag.name}
							className="flex items-center gap-2 flex-row w-full pl-12 text-blue-500"
						>
							<Link href={`/tag/${tag.name}`}>
								<div>#{tag.name}</div>
							</Link>
						</div>
					))
				) : (
					<div>No tags found</div>
				)}
			</div>
		</div>
	)
}
