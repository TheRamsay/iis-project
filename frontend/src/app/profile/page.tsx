import { notFound, redirect } from "next/navigation";
import { ProfilePage } from "./_ui/profile-page";
import { cookies } from "next/headers";

export default function Page() {
	const cookiez = cookies();

	const user_id = "my_user";

	if (!user_id) {
		return redirect("/login");
	}

	return <ProfilePage profile_id={user_id} />;
}
