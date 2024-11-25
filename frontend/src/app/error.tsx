'use client'

export default function ErrorPage({ error }: { error: Error }) {
	return (
		<div className="w-full flex justify-center pt-8 text-3xl">
			{error.message}
		</div>
	)
}
