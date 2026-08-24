<script lang="ts">
	import { resolve } from '$app/paths';
	import HeaderLink from '$c/header/header-link.svelte';
	import icon from '$lib/assets/icon.svg';
	import { str } from '$lib/lang';
</script>

<header
	class={[
		'flex',
		'w-full',
		'h-6',
		'px-1.25',
		'py-1',
		'text-sm',
		'font-medium',
		'text-muted-foreground'
	]}
>
	<div class={['flex', 'basis-1/3', 'justify-start', 'items-center', 'gap-2.5', 'h-full']}>
		<span class={['text-foreground', 'h-full', 'flex', 'items-center', 'group', 'cursor-pointer']}>
			<img
				src={icon}
				alt="Icon"
				class={[
					'h-full',
					'mr-0.5',
					'dark:mr-1',
					'rounded-xs',
					'border-transparent',
					'dark:border-border',
					'border-2',
					'rotate-0',
					'group-hover:rotate-360',
					'transition-all'
				]}
			/>
			<span
				class={[
					'group-hover:decoration-muted-foreground',
					'transition-all',
					'decoration-transparent',
					'underline'
				]}
			>
				<span class={['font-bold']}>Surreal</span>
				<span class={['font-normal']}>Nyvo</span>
			</span>
		</span>
		<HeaderLink href={resolve('/')}>{str('action.file')}</HeaderLink>
		<HeaderLink href="/">{str('action.edit')}</HeaderLink>
		<HeaderLink href="/">{str('action.view')}</HeaderLink>
	</div>
	<div class={['basis-1/3', 'flex', 'h-full', 'justify-start', 'items-center', 'font-mono']}>
		search...
	</div>
	<div class={['basis-1/3']}>
		<!-- idk what to put here, normally window icons would be here -->
	</div>
</header>
