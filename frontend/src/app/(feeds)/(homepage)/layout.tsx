import { Container } from "@/components/components/container";
import type React from "react";

export default function Layout({ children }: { children: React.ReactNode }) {
	return (
		<Container maxWidth="md" className="pt-4">
			{children}
		</Container>
	);
}
