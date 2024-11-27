<script lang="ts">
	import Prism from 'prismjs';
	import { onMount } from 'svelte';

	onMount(() => {
		Prism.highlightAll();
	});
</script>

<article class="prose-gray prose-base text-white">
	<h1>JavaScript/TypeScript Documentation</h1>
	<a
		href="https://www.npmjs.com/package/fitl-js"
		class="inline-flex items-center font-medium text-blue-600 dark:text-blue-500 hover:underline"
	>
		<h2>fitl-js is available on npm repositories</h2></a
	>

	<pre class="codeblock language-bash"><code>
{`npm install fitl-js`}

{'pnpm install fitl-js'}
    </code></pre>

	<h2>Getting Started</h2>

	<p>To filter a basic JavaScript array of objects:</p>

	<pre class="codeblock language-javascript"><code>
{`import { fitlFilter } from "fitl-js";

let tableData = [
    { category: "meat" },
    { category: "fruit" }
];

let query = "category = fruit";

async function main() {
    try {
        let resultTable = await fitlFilter(query, tableData);
        console.log(resultTable);
    } catch (error: unknown) {
        console.error(error);
    }
}
main();
`}
</code></pre>

	<p>With everything installed correctly this outputs to a new table:</p>
	<pre class="codeblock language-javascript"><code>
{'[{category: "fruit"}]'}
    </code></pre>

	<p>
		Options are optional of course, currently used to specify input/output table types with other
		future options coming soon.
	</p>

	<pre class="codeblock language-javascript"><code>
{`import  { fitlFilter, type Options } from 'fitl-js';

let tableData = [
    { category: "meat" },
    { category: "fruit" }
];

let query = "category = fruit";

let options: Options = { tableFormat: 'JSARRAY' };

async function main(){
    try {
        let resultTable = await fitlFilter(query, tableData, options);
    } catch (error: unknown) {
        console.error(error);
    }
}
main();`}
    </code></pre>

	<p>
		Default tableFormat is JSARRAY, other table formats coming soon and will have to be specifically
		defined in options
	</p>
</article>

<style>
	article {
		margin-left: 10%;
		width: 80%;
		margin-right: 10%;
	}

	.codeblock {
		/* background-color: black; */
		/* color: whitesmoke; */
		padding-top: 0;
		padding-bottom: 0;
	}

	table {
		border-collapse: collapse;
		overflow: hidden;
		font-size: 0.85rem;
		color: white;
	}

	th,
	td {
		padding: 10px;
		background-color: rgb(26, 26, 26);
		color: whitesmoke;
		border: 1px black solid;
		border-bottom: 1px whitesmoke solid;
	}

	/* th {
		text-align: center;
	} */

	@media (max-width: 1280px) {
		article {
			margin-left: 0px;
		}
	}

	@media (max-width: 768px) {
		/* @media (max-width: 1280px) { */
		article {
			margin-left: 0px;
		}
	}
</style>
