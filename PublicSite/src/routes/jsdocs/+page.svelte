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
{'> [{category: "fruit"}]'}
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

	<h2>Column Types</h2>
	<p>You can specify a data type for a column for more specific query options.</p>

	<p>For example:</p>

	<pre class="codeblock language-javascript"><code>
{`let tableData = [
    { name: "apple", amount: 3 },
    { name: "banana", amount: 8 }
];
let query = "";

console.log(await fitlFilter(query, tableData));
`}
</code></pre>

	<p>The above will automatically parse the "amount" column as a string. This example outputs:</p>

	<pre class="codeblock language-javascript"><code>
{`  [{ name: "apple", amount: 3 },
   { name: "banana", amount: 8 }]`}
    </code></pre>

	<p>
		And only allows you to do string based operations on the amount column. To specify that the
		amount column is a numeric column in the options parameter of "filtFilter" like so:
	</p>
	<pre class="codeblock language-javascript"><code>
{`const options: Options = {
    columnTypes: {
        amount: "number",
    }
}
`}
</code></pre>

	<p>In code example:</p>

	<pre class="codeblock language-javascript"><code>
{`let tableData = [
    { name: "apple", amount: 3 },
    { name: "banana", amount: 8 }
];
let query = "";

const options: Options = {
    tableFormat: "JSARRAY",
    columnTypes: {
        amount: "number",
    }
}

console.log(await fitlFilter(query, tableData, options));
`}
</code></pre>

	<p>
		Which allows for numeric operations on columns and outputs the "amount" values as actual
		JavaScript numbers:
	</p>
	<pre class="codeblock language-javascript"><code>
{`  [{ name: "apple", amount: 3 },
   { name: "banana", amount: 8 }]`}
    </code></pre>


</article>

<style>
	article {
		width: 80%;
	}

	.codeblock {
		padding-top: 0;
		padding-bottom: 0;
	}

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
