import { NavigationContainer } from '@/components/components/navigation-container'
import { HeaderProfile } from './header-profile/header-profile'
import { HeaderSearch } from './header-search'
import { HeaderNavigation } from './header-navigation'

export function Header() {
	return (
		<NavigationContainer>
			<div className="flex justify-between w-full items-center">
				<div className="flex-1">
					<HeaderNavigation />
				</div>
				<div className="absolute left-1/2 transform -translate-x-1/2">
					<HeaderSearch />
				</div>
				<div className="flex-1 text-right">
					<HeaderProfile />
				</div>
			</div>
		</NavigationContainer>
	)
}
