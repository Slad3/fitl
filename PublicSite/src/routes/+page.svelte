<script lang="ts">
	import { base } from '$app/paths';
	import FoodExample from '$lib/components/FoodExample.svelte';

	import ExampleGif from '$lib/images/example.gif';

	// import { initFlowbite } from 'flowbite';
	// import { onMount } from 'svelte';

	// onMount(() => {
	// 	initFlowbite();
	// });
</script>

<article class="prose-gray prose-base text-white">
	<div class=" border-white mb-10">
		<h1>FiTL (Filter Table Language)</h1>
		<h3>A simple language for filtering tables, because sometimes search algorithms just suck.</h3>
		<!-- <a href="{base}/food"> -->
		<a href="#whyNotSQL">
			<img src={ExampleGif} alt="Example Query" />
		</a>
	</div>

	<ul class="list-inside border-b border-white pb-5">
		<li>
			<h2>
				Learn more about <a
					href="{base}/queries"
					class="inline-flex items-center font-medium text-blue-600
					dark:text-blue-500 hover:underline"
				>
					How to Write Queries
					<svg
						class="w-4 h-4 ms-2 rtl:rotate-180"
						aria-hidden="true"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 14 10"
					>
						<path
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M1 5h12m0 0L9 1m4 4L9 9"
						/>
					</svg>
				</a>
			</h2>
		</li>
		<li>
			<h2>
				Check out an interactive example with example queries on
				<a
					href="{base}/playlist"
					class="inline-flex items-center font-medium text-blue-600 dark:text-blue-500 hover:underline"
				>
					Playlist filter
					<svg
						class="w-4 h-4 ms-2 rtl:rotate-180"
						aria-hidden="true"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 14 10"
					>
						<path
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M1 5h12m0 0L9 1m4 4L9 9"
						/>
					</svg>
				</a>
			</h2>
		</li>
		<li>
			<h2>
				JavaScript (fitl-js)
				<a
					href="{base}/jsdocs"
					class="inline-flex items-center font-medium text-blue-600 dark:text-blue-500 hover:underline"
				>
					Installation and getting started
					<svg
						class="w-4 h-4 ms-2 rtl:rotate-180"
						aria-hidden="true"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 14 10"
					>
						<path
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M1 5h12m0 0L9 1m4 4L9 9"
						/>
					</svg>
				</a>
			</h2>
		</li>

		<li>
			<h2>
				Written for Rust and JavaScript <a
					href="https://github.com/Slad3/FilterTableLanguage"
					class="inline-flex items-center font-medium text-blue-600 dark:text-blue-500 hover:underline"
				>
					Check out source code on GitHub
					<svg
						class="w-4 h-4 ms-2 rtl:rotate-180"
						aria-hidden="true"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 14 10"
					>
						<path
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M1 5h12m0 0L9 1m4 4L9 9"
						/>
					</svg>
				</a>
			</h2>
		</li>
	</ul>

	<h2>What is FiTL?</h2>
	<p>
		FiTL (Filter Table Language) is a simple query langauge allowing end users to quickly and
		specifically filter rows from a table. Users filter rows from tables based on columns and
		conditional statements for those columns. Multiple conditional statemens can be combined/chained
		to allow for further specificity of table results. Designed to be easy to implement for
		developers, easy to learn and remember for general users, but also allows room to master for
		power users.
	</p>
	<h2>Why did I, specifically, create FiTL?</h2>
	<p>
		Because Spotify and Genius.com's search algorithm really freaking sucks. Learn more <a
			href="https://github.com/Slad3/FilterTableLanguage"
			class="inline-flex items-center font-medium text-blue-600 dark:text-blue-500 hover:underline"
		>
			here
			<svg
				class="w-4 h-4 ms-2 rtl:rotate-180"
				aria-hidden="true"
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 14 10"
			>
				<path
					stroke="currentColor"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M1 5h12m0 0L9 1m4 4L9 9"
				/>
			</svg>
		</a>
	</p>

	<h2>Real world potential implementations include:</h2>
	<ul class="space-y-1 list-outside list-disc">
		<li>Music Playlists (IE: Spotify Liked songs or an already made playlist)</li>
		<li>
			Search Page on Product pages (whether in blank search page or just filtering down search
			results)
		</li>
		<li>Movie/Video search results (Netflix/Hulu/IMDB searching/filtering)</li>
	</ul>

	<div id="whyNotSQL">
		<h2>Why not just use SQL? (or a modified version of sql)</h2>

		<ul class=" space-y-1 list-disc list-outside">
			<li>
				FiTL is designed to be easier to type in a simple text box, especially on mobile devices
			</li>
			<li>
				SQL is easy enough to learn, but FiTL is made to be easy enough for people who touch grass
			</li>
			<li>SQL allows for table joining while FiTl is limited to singular table interacitons</li>
			<li>
				In general FiTL is meant for quickly filtering an existing table for end user use while SQL
				is meant for more reusable structured queries. Although neither is contrained to previously
				said purposes.
			</li>
		</ul>
	</div>

	<div class="py-5 mt-5 border-t border-white" id="foodExample" style="min-height: 800px;">
		<h1>Interactive Example</h1>
		<p>
			Here's an interactive example with a table of food. Feel free to try out any of the example
			queries below or to write your own queries.
		</p>
		<p>
			Learn more about writing queries and advanced query tricks <a
				href="{base}/queries"
				class="inline-flex items-center font-medium text-blue-600
					dark:text-blue-500 hover:underline"
			>
				here
				<svg
					class="w-4 h-4 ms-2 rtl:rotate-180"
					aria-hidden="true"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 14 10"
				>
					<path
						stroke="currentColor"
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M1 5h12m0 0L9 1m4 4L9 9"
					/>
				</svg>
			</a>
		</p>
		<FoodExample />

		<!-- <DataTable data={foodData} order={['name', 'category', 'color']} /> -->
	</div>
</article>

<style>
</style>
