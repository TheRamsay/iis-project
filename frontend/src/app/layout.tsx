import type { Metadata } from 'next'
import localFont from 'next/font/local'
import './globals.css'
import '../components/index.css'
import { Header } from './_ui/header/header'
import { Providers } from './providers'
import { getSession } from './_lib/auth/get-session'

const geistSans = localFont({
	src: './_fonts/GeistVF.woff',
	variable: '--font-geist-sans',
	weight: '100 900',
})
const geistMono = localFont({
	src: './_fonts/GeistMonoVF.woff',
	variable: '--font-geist-mono',
	weight: '100 900',
})

export const metadata: Metadata = {
	title: 'FITstagram',
}

export default async function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode
}>) {
	const session = await getSession()

	return (
		<html
			lang="en"
			className="[color-scheme:dark] dark"
			suppressHydrationWarning
		>
			<body
				className={`${geistSans.variable} ${geistMono.variable} antialiased h-screen w-full flex flex-col`}
			>
				<Providers session={session}>
					<Header />
					<div className="h-full">{children}</div>
				</Providers>
			</body>
		</html>
	)
}
