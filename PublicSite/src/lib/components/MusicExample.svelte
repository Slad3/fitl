<script lang="ts">
	import '$lib/stylesheets/fitlExamples.css';
	import { onMount } from 'svelte';

	import DataTable from '$lib/components/DataTable.svelte';
	import { songsTable } from '$lib/data/musicData';

	import { fitlFilter } from 'fitl-js';

	let queryTextBox: any;
	let query = '';
	let result_table: Array<Object> = [];

	onMount(async () => {
		result_table = songsTable;
		queryTextBox = document.getElementById('query');
	});

	function colorQueryBox(input: boolean) {
		queryTextBox.style.borderColor = input ? 'green' : 'red';
	}

	async function onQueryChange(event: { target: any }) {
		query = event.target.value;
		try {
			let temp = await fitlFilter(query, songsTable);
			result_table = temp;

			colorQueryBox(true);
		} catch (error: any) {
			colorQueryBox(false);
			console.error(error);
		}
	}

	function example1() {
		query = 'artist =: erykah';
		onQueryChange({ target: { value: query } });
	}

	function example2() {
		query = 'album =: the';
		onQueryChange({ target: { value: query } });
	}

	function example3() {
		query = 'artist = "black thought" | artist =: roots';
		onQueryChange({ target: { value: query } });
	}
</script>

<article>
	<span class="form__group field">
		<input
			type="text"
			id="query"
			name="query"
			placeholder="Query"
			class="form__field"
			bind:value={query}
			on:input={onQueryChange}
			required
		/>
		<label for="query" class="form__label">Query</label>
	</span>
	<br />
	<br />
	<div class="examples">
		<p class="examplesTitle">Example Queries</p>
		<button class="filter_button example_button" on:click={example1}>All Erykah Badu Songs</button>
		<button class="filter_button example_button" on:click={example2}>All Albums with "the"</button>
		<button class="filter_button example_button" on:click={example3}>All Black Thought Songs</button
		>
	</div>
	<span id="tableInfo">
		<div class="tableSizeInfo">
			Current Table Length: {result_table.length} | Original Table Length: {songsTable.length}
		</div></span
	>
	<DataTable data={result_table} order={['title', 'artist', 'album']}></DataTable>
</article>

<style>
	article {
		text-align: center;
		padding: 1em;
		width: 100%;
		margin: 0 auto;
	}
	.example_button {
		width: 30%;
	}
</style>
