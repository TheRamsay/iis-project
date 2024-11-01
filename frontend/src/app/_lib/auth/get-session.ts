import { cookies } from 'next/headers'
import type { Session } from './types'

export async function getSession(): Promise<Session | null> {
	const cookiez = cookies()
	const session = cookiez.get('session')

	// if (!session) {
	// 	return null
	// }

	return {
		userId: 'uuiduuiduuid',
		username: 'fitstagram',
		avatar: {
			src: 'https://avatars.githubusercontent.com/u/7655549?v=4',
			width: 128,
			height: 128,
		},
		role: 'regular',
		expires: Date.now() + 1000 * 60 * 60 * 24,
	}
}
