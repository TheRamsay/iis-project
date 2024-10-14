import { HomeIcon } from "lucide-react";
import Link from "next/link";

export function HeaderNavigation() {
	return (
		<div className="flex flex-row gap-4">
			<Link href="/">
				<HomeIcon width={24} height={24} />
			</Link>
			{/* <Link href="/">
				<HomeIcon width={24} height={24} />
			</Link> */}
		</div>
	);
}
