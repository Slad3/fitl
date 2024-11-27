<script lang="ts">
</script>

<article class="prose-gray prose-base text-white">
	<h1>JavaScript/TypeScript Documentation</h1>

	<h2>Installation</h2>
	<p>fitl-js is available on npm repositories</p>
	<code> pnpm install fitl-js </code>

	<h2>Hello World</h2>

	<div>
		<h1>Example Code</h1>
		<pre class="codeblock">
            {`import { fitl_filter } from "fitl-js";

            let tableData = [
                { category: "meat" },
                { category: "fruit" }
            ];

            let query = "category = fruit";

            async function main() {
                try {
                    let resultTable = await fitl_filter(query, tableData);
                } catch (error: unknown) {
                    console.error(error);
                }
            }
            main();`}
    </pre>
	</div>
</article>

<style>
	article {
		margin-left: 10%;
		width: 80%;
		margin-right: 10%;
	}

	.codeblock {
		background-color: black;
		/* padding: 10px; */
		/* padding-left: 20px; */
		/* padding-right: 20px; */
		border-radius: 7px;
		color: whitesmoke;
	}

	h1,
	h2,
	h3 {
		/* text-align: left; */
		color: whitesmoke;
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
