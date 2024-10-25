import classNames from 'classnames'

export function formClassnames(
	{
		isDirty = false,
		isError = false,
	}: { isDirty?: boolean; isError?: boolean },
	...rest: classNames.ArgumentArray
) {
	return classNames(
		...rest,
		'!border border-transparent',
		isDirty && '!border-blue-500 border-opacity-40',
		isError && '!border-red-500 border-opacity-40',
	)
}
