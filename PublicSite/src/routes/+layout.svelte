<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { base } from '$app/paths';
	import githubLogo from '$lib/images/icons/githubLogo.png';
	import fitlLogo from '$lib/images/icons/favicon-32x32.png';
	import { onMount } from 'svelte';

	const transformAttribute = 'transform-none';
	let exampleDropdownShown = true;
	let docsDropdownShown = true;
	let drawerNav: HTMLElement;

	onMount(() => {
		drawerNav = document.getElementById('drawer-navigation')!;
	});

	function toggleSidebar() {
		if (drawerNav.classList.contains(transformAttribute)) {
			drawerNav.classList.remove(transformAttribute);
		} else {
			drawerNav.classList.add(transformAttribute);
		}
		return null;
	}
</script>

<div class="antialiased bg-primary-background">
	<nav class="px-4 py-2.5 bg-secondary-background border-black fixed left-0 right-0 top-0 z-50">
		<div class="flex flex-wrap justify-between items-center">
			<div class="flex justify-start items-center">
				<button
					data-drawer-target="drawer-navigation"
					data-drawer-toggle="drawer-navigation"
					aria-controls="drawer-navigation"
					class="p-2 mr-2 text-gray-600 rounded-lg cursor-pointer md:hidden hover:text-gray-900 hover:bg-gray-100 focus:bg-gray-100 dark:focus:bg-gray-700 focus:ring-2 focus:ring-gray-100 dark:focus:ring-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
					on:click={toggleSidebar}
				>
					<svg
						aria-hidden="true"
						class="w-6 h-6"
						fill="currentColor"
						viewBox="0 0 20 20"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							fill-rule="evenodd"
							d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h6a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
							clip-rule="evenodd"
						></path>
					</svg>
					<svg
						aria-hidden="true"
						class="hidden w-6 h-6"
						fill="currentColor"
						viewBox="0 0 20 20"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							fill-rule="evenodd"
							d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
							clip-rule="evenodd"
						></path>
					</svg>
					<span class="sr-only">Toggle sidebar</span>
				</button>
				<a href="https://fitl.benbarcaskey.com" class="flex items-center justify-between mr-4">
					<img src={fitlLogo} alt="" class="h-10 w-10 mr-3" />
					<span class="self-center text-2xl font-semibold whitespace-nowrap text-white text-center"
						>Filter Table Language</span
					>
				</a>
			</div>
			<div class="flex items-center lg:order-2"></div>
		</div>
	</nav>

	<aside
		class="fixed top-0 left-0 z-40 w-64 h-screen pt-14 transition-transform -translate-x-full md:translate-x-0 bg-secondary-background border-gray-800"
		aria-label="drawer-navigation"
		id="drawer-navigation"
	>
		<div class="overflow-y-auto py-5 px-3 h-full bg-secondary-background">
			<ul class="space-y-2">
				<li>
					<a
						href="/"
						class="flex items-center p-2 text-base font-medium rounded-lg text-white hover:bg-hover-accent group"
					>
						<svg
							class="w-6 h-6 text-gray-800 dark:text-white"
							aria-hidden="true"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							fill="none"
							viewBox="0 0 24 24"
						>
							<path
								stroke="currentColor"
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="m4 12 8-8 8 8M6 10.5V19a1 1 0 0 0 1 1h3v-3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v3h3a1 1 0 0 0 1-1v-8.5"
							/>
						</svg>

						<span class="ml-3">Home</span>
					</a>
				</li>
				<li>
					<a
						href="/queries"
						class="flex items-center p-2 text-base font-medium rounded-lg text-white hover:bg-hover-accent group"
					>
						<svg
							class="w-6 h-6 text-gray-800 dark:text-white"
							aria-hidden="true"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							fill="none"
							viewBox="0 0 24 24"
						>
							<path
								stroke="currentColor"
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9.529 9.988a2.502 2.502 0 1 1 5 .191A2.441 2.441 0 0 1 12 12.582V14m-.01 3.008H12M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
							/>
						</svg>

						<span class="ml-3">How to Write Queries</span>
					</a>
				</li>
				<li>
					<!-- Example Dropdown -->
					<button
						type="button"
						class="flex items-center p-2 w-full text-base font-medium rounded-lg transition duration-75 group text-white hover:bg-hover-accent"
						aria-controls="dropdown-pages"
						data-collapse-toggle="dropdown-pages"
						on:click={() => {
							exampleDropdownShown = !exampleDropdownShown;
						}}
					>
						<svg
							aria-hidden="true"
							class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								fill-rule="evenodd"
								d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z"
								clip-rule="evenodd"
							></path>
						</svg>
						<span class="flex-1 ml-3 text-left whitespace-nowrap">Examples</span>
						<svg
							aria-hidden="true"
							class="w-6 h-6"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								fill-rule="evenodd"
								d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
								clip-rule="evenodd"
							></path>
						</svg>
					</button>
					{#if exampleDropdownShown}
						<ul id="dropdown-pages" class="py-2 space-y-2">
							<li aria-current={$page.url.pathname === '/playlist' ? 'page' : undefined}>
								<a
									href="{base}/playlist"
									class="flex items-center p-2 pl-11 w-full text-base font-medium text-gray-900 rounded-lg transition duration-75 group dark:text-white hover:bg-hover-accent"
								>
									<svg
										class="w-6 h-6 text-gray-800 dark:text-white"
										aria-hidden="true"
										xmlns="http://www.w3.org/2000/svg"
										width="24"
										height="24"
										fill="none"
										viewBox="0 0 24 24"
									>
										<path
											stroke="currentColor"
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M17 15.5V5s3 1 3 4m-7-3H4m9 4H4m4 4H4m13 2.4c0 1.326-1.343 2.4-3 2.4s-3-1.075-3-2.4 1.343-2.4 3-2.4 3 1.075 3 2.4Z"
										/>
									</svg>

									<span class="ml-3">Playlist Filter</span></a
								>
							</li>
							<li aria-current={$page.url.pathname === '/food' ? 'page' : undefined}>
								<a
									href="{base}/food"
									class="flex items-center p-2 pl-11 w-full text-base font-medium text-gray-900 rounded-lg transition duration-75 group dark:text-white hover:bg-hover-accent"
								>
									<svg
										class="w-6 h-6 text-gray-800 dark:text-white"
										aria-hidden="true"
										xmlns="http://www.w3.org/2000/svg"
										width="24"
										height="24"
										fill="none"
										viewBox="0 0 24 24"
									>
										<path
											stroke="currentColor"
											stroke-linecap="round"
											stroke-width="2"
											d="M4.37 7.657c2.063.528 2.396 2.806 3.202 3.87 1.07 1.413 2.075 1.228 3.192 2.644 1.805 2.289 1.312 5.705 1.312 6.705M20 15h-1a4 4 0 0 0-4 4v1M8.587 3.992c0 .822.112 1.886 1.515 2.58 1.402.693 2.918.351 2.918 2.334 0 .276 0 2.008 1.972 2.008 2.026.031 2.026-1.678 2.026-2.008 0-.65.527-.9 1.177-.9H20M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
										/>
									</svg>

									<span class="ml-3">Food Filter</span></a
								>
							</li>
							<!-- <li>
							<a
								href="/playlist"
								class="flex items-center p-2 pl-11 w-full text-base font-medium rounded-lg transition duration-75 group text-white hover:bg-hover-accent"
								>Product Page (Cars)</a
							>
						</li> -->
						</ul>
					{/if}
				</li>
				<li>
					<!-- Docs Dropdown -->
					<button
						type="button"
						class="flex items-center p-2 w-full text-base font-medium rounded-lg transition duration-75 group text-white hover:bg-hover-accent"
						aria-controls="dropdown-pages"
						data-collapse-toggle="dropdown-pages"
						on:click={() => {
							docsDropdownShown = !docsDropdownShown;
						}}
					>
						<svg
							aria-hidden="true"
							class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								fill-rule="evenodd"
								d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z"
								clip-rule="evenodd"
							></path>
						</svg>
						<span class="flex-1 ml-3 text-left whitespace-nowrap">Docs</span>
						<svg
							aria-hidden="true"
							class="w-6 h-6"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								fill-rule="evenodd"
								d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
								clip-rule="evenodd"
							></path>
						</svg>
					</button>
					{#if docsDropdownShown}
						<ul id="dropdown-pages" class="py-2 space-y-2">
							<li aria-current={$page.url.pathname === '/jsdocs' ? 'page' : undefined}>
								<a
									href="{base}/jsdocs"
									class="flex items-center p-2 pl-11 w-full text-base font-medium text-gray-900 rounded-lg transition duration-75 group dark:text-white hover:bg-hover-accent"
								>
									<svg
										aria-hidden="true"
										class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
										fill="currentColor"
										viewBox="0 0 20 20"
										xmlns="http://www.w3.org/2000/svg"
									>
										<path
											fill-rule="evenodd"
											d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z"
											clip-rule="evenodd"
										></path>
									</svg>

									<span class="ml-3">JS Getting Started</span></a
								>
							</li>
							<!-- <li aria-current={$page.url.pathname === '/food' ? 'page' : undefined}>
								<a
									href="{base}/food"
									class="flex items-center p-2 pl-11 w-full text-base font-medium text-gray-900 rounded-lg transition duration-75 group dark:text-white hover:bg-hover-accent"
								>
									<svg
										aria-hidden="true"
										class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
										fill="currentColor"
										viewBox="0 0 20 20"
										xmlns="http://www.w3.org/2000/svg"
									>
										<path
											fill-rule="evenodd"
											d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z"
											clip-rule="evenodd"
										></path>
									</svg>

									<span class="ml-3">Rust Docs</span></a
								>
							</li> -->
						</ul>
					{/if}
				</li>
				<li>
					<a
						href="/about"
						class="flex items-center p-2 text-base font-medium rounded-lg text-white hover:bg-hover-accent group"
					>
						<svg
							class="w-6 h-6 text-gray-800 dark:text-white"
							aria-hidden="true"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							fill="none"
							viewBox="0 0 24 24"
						>
							<path
								stroke="currentColor"
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9.529 9.988a2.502 2.502 0 1 1 5 .191A2.441 2.441 0 0 1 12 12.582V14m-.01 3.008H12M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
							/>
						</svg>

						<span class="ml-3">About/Why FiTL</span>
					</a>
				</li>
			</ul>
			<!-- <ul class="pt-5 mt-5 space-y-2 border-t border-gray-200 dark:border-gray-700">
				<li>
					<a
						href="#"
						class="flex items-center p-2 text-base font-medium text-gray-900 rounded-lg transition duration-75 hover:bg-gray-100 dark:hover:bg-gray-700 dark:text-white group"
					>
						<svg
							aria-hidden="true"
							class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z"></path>
							<path
								fill-rule="evenodd"
								d="M4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3zm-3 4a1 1 0 100 2h.01a1 1 0 100-2H7zm3 0a1 1 0 100 2h3a1 1 0 100-2h-3z"
								clip-rule="evenodd"
							></path>
						</svg>
						<span class="ml-3">Docs</span>
					</a>
				</li>
				<li>
					<a
						href=""
						class="flex items-center p-2 text-base font-medium text-gray-900 rounded-lg transition duration-75 hover:bg-gray-100 dark:hover:bg-gray-700 dark:text-white group"
					>
						<svg
							aria-hidden="true"
							class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								d="M7 3a1 1 0 000 2h6a1 1 0 100-2H7zM4 7a1 1 0 011-1h10a1 1 0 110 2H5a1 1 0 01-1-1zM2 11a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4z"
							></path>
						</svg>
						<span class="ml-3">Components</span>
					</a>
				</li>
				<li>
					<a
						href=""
						class="flex items-center p-2 text-base font-medium text-gray-900 rounded-lg transition duration-75 hover:bg-gray-100 dark:hover:bg-gray-700 dark:text-white group"
					>
						<svg
							aria-hidden="true"
							class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
							fill="currentColor"
							viewBox="0 0 20 20"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								fill-rule="evenodd"
								d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-2 0c0 .993-.241 1.929-.668 2.754l-1.524-1.525a3.997 3.997 0 00.078-2.183l1.562-1.562C15.802 8.249 16 9.1 16 10zm-5.165 3.913l1.58 1.58A5.98 5.98 0 0110 16a5.976 5.976 0 01-2.516-.552l1.562-1.562a4.006 4.006 0 001.789.027zm-4.677-2.796a4.002 4.002 0 01-.041-2.08l-.08.08-1.53-1.533A5.98 5.98 0 004 10c0 .954.223 1.856.619 2.657l1.54-1.54zm1.088-6.45A5.974 5.974 0 0110 4c.954 0 1.856.223 2.657.619l-1.54 1.54a4.002 4.002 0 00-2.346.033L7.246 4.668zM12 10a2 2 0 11-4 0 2 2 0 014 0z"
								clip-rule="evenodd"
							></path>
						</svg>
						<span class="ml-3">Help</span>
					</a>
				</li>
			</ul> -->
		</div>
		<div
			class="absolute bottom-0 left-0 justify-center p-4 space-x-4 w-full lg:flex dark:bg-secondary-background z-20"
		>
			<a
				type="button"
				class="inline-flex justify-center p-2 rounded cursor-pointer hover:text-white text-gray-400 hover:bg-hover-accent box-content h-12 w-12"
				href="https://github.com/Slad3/fitl"
			>
				<img src={githubLogo} alt="GitHub" class="githubLogo" />
			</a>
		</div>
	</aside>

	<main class="p-4 md:ml-64 h-auto pt-20 bg-primary-background text-white">
		<slot />
	</main>
</div>

<style>
	main {
		min-height: 100vh;
		max-width: 800px;
	}
</style>
