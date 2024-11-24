import { Container } from '@/components/components/container'

export default function Layout({ children }: { children: React.ReactNode }) {
	return (
		<Container maxWidth="xl" className="py-8">
			{children}
		</Container>
	)
}
