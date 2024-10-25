import type { Metadata } from 'next'
import localFont from 'next/font/local'
import './globals.css'
import '../components/index.css'
import { Header } from './_ui/header/header'
7777
import { Providers } from './providers'

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

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode
}>) {
	return (
		<html
			lang="en"
			className="[color-scheme:dark] dark"
			suppressHydrationWarning
		>
			<body
				className={`${geistSans.variable} ${geistMono.variable} antialiased h-screen w-full flex flex-col`}
			>
				<Providers>
					<Header />
					<div className="h-full">{children}</div>
				</Providers>
			</body>
		</html>
	)
}
