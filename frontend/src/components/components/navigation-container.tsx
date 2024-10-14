import { type VariantProps, cva } from "class-variance-authority";

export const navigationContainerVariants = cva(
	"px-4 sticky flex items-center flex-grow gap-4 top-0 z-50 min-h-[56px] max-h-[56px] h-[56px]",
	{
		variants: {
			variant: {
				default: "border-b border-gray-200 dark:border-gray-700 bg-background",
				transparent: "",
			},
		},
		defaultVariants: {
			variant: "default",
		},
	},
);

interface NavContainerProps
	extends VariantProps<typeof navigationContainerVariants> {
	children: React.ReactNode;
}

export const NavigationContainer: React.FC<NavContainerProps> = ({
	children,
	variant,
}) => {
	return (
		<div className={navigationContainerVariants({ variant })}>
			<div className="flex items-center justify-between flex-grow gap-4">
				{children}
			</div>
		</div>
	);
};
