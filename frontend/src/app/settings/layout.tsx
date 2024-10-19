import { Container } from '@/components/components/container'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'

export default function Layout({ children }: { children: React.ReactNode }) {
	const cookiez = cookies()

	const user_id = 1231

	if (!user_id) {
		return redirect('/')
	}

	return <Container maxWidth="2xl">{children}</Container>
}
