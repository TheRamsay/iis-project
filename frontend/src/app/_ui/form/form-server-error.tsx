interface FormServerError {
	error: Error | null
}

export function FormServerError({ error }: FormServerError) {
	if (!error) {
		return null
	}

	return (
		<div className="w-full text-red-500 p-4 border-red-500 border bg-red-500 bg-opacity-30 rounded-xl">
			{error?.message}
		</div>
	)
}
