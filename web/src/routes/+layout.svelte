<script lang="ts">
	import '$style/main.css';
	import icon from '$lib/assets/icon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import Header from '$c/header.svelte';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={icon} /></svelte:head>

<ModeWatcher />

<Header />

<main class={['h-full', 'w-full', 'min-h-[calc(100dvh-1.5rem)]', 'flex', 'flex-col']}>
	{@render children()}
</main>
