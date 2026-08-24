<script lang="ts">
	let {
		href,
		children
	}: {
		href: string;
		children: () => any;
	} = $props();
</script>

<a {href} class={['transition-all', 'hover:text-foreground', 'cursor-pointer']}>
	{@render children()}
</a>
