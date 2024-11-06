<script lang="ts">
	import './../../app.css';
	import { onMount } from 'svelte';

	import DataTable from '$lib/components/DataTable.svelte';
	import { songsTable } from '../../lib/data/musicData.js';
	import { cleanMap, mapToObj, printError } from '$lib/utils.ts';

	import init, { fitl_filter } from 'fitl-wasm';
	import Help from '../../lib/components/Help.svelte';

	let queryTextBox: any;
	let query = '';
	let result_table: Array<Object> = [];
	const order = ['title', 'artist', 'album'];

	let showModal = false;

	onMount(async () => {
		await init();
		result_table = songsTable;
		queryTextBox = document.getElementById('query');
	});

	function colorQueryBox(input: boolean) {
		queryTextBox.style.borderColor = input ? 'green' : 'red';
	}

	function onQueryChange(event: { target: any }) {
		query = event.target.value;
		try {
			let temp = cleanMap(fitl_filter(query, songsTable));
			result_table = temp;

			colorQueryBox(true);
		} catch (error: { error: { level: string; details: string } }) {
			printError(cleanMap(error));
			colorQueryBox(false);
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

	function closeModal() {
		showModal = false;
	}

	function toggleModal() {
		showModal = !showModal;
	}
</script>

<main>
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
	<button class="filter_button example_button" on:click={example1}>All Erykah Badu Songs</button>
	<button class="filter_button example_button" on:click={example2}>All Albums with "the"</button>
	<button class="filter_button example_button" on:click={example3}>All Black Thought Songs</button>
	<br />
	<span id="tableInfo">
		<div class="tableSizeInfo">
			Current Table Length: {result_table.length} | Original Table Length: {songsTable.length}
		</div></span
	>
	<DataTable data={result_table} {order}></DataTable>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		width: 100%;
		/* max-width: 800px; */
		margin: 0 auto;
	}

	:root {
		--primary: #9b9b9b;
		--secondary: red;
		--white: #fff;
		--gray: #9b9b9b;
	}

	.form__group {
		position: relative;
		padding: 15px 0 0;
		margin-top: 10px;
		width: 100%;
	}

	.form__field {
		font-family: inherit;
		/* width: 80%; */
		width: 100%;
		border: 0;
		border-bottom: 2px solid var(--gray);
		outline: 0;
		font-size: 1.3rem;
		color: var(--white);
		padding: 7px 0;
		background: transparent;
		transition: border-color 0.2s;
	}

	.form__field::placeholder {
		color: transparent;
	}

	.form__field:placeholder-shown ~ .form__label {
		font-size: 1.3rem;
		cursor: text;
		top: 10px;
	}

	.form__label {
		position: absolute;
		top: 0;
		display: block;
		transition: 0.2s;
		font-size: 0.8rem;
		color: var(--gray);
	}

	.form__field:focus ~ .form__label {
		position: absolute;
		top: 0;
		display: block;
		transition: 0.2s;
		font-size: 1rem;
		color: var(--primary);
		font-weight: 700;
	}

	.form__field:focus {
		padding-bottom: 6px;
		font-weight: 700;
		border-width: 3px;
		/* border-image: red; */
		border-color: gray;
		border-image-slice: 1;
	}

	.form__field:required,
	.form__field:invalid {
		box-shadow: none;
	}

	.filter_button {
		width: 19%;
		height: 100%;
		font-size: 1.3rem;
		background-color: #2c2c2c;
		border-radius: 4px;
		border-style: none;
		box-sizing: border-box;
		color: #fff;
		cursor: pointer;
		display: inline-block;
		font-family: 'Farfetch Basis', 'Helvetica Neue', Arial, sans-serif;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.5;
		margin: 0;
		max-width: none;
		min-height: 44px;
		min-width: 10px;
		outline: none;
		overflow: hidden;
		padding: 9px 20px 8px;
		position: relative;
		text-align: center;
		text-transform: none;
		user-select: none;
		-webkit-user-select: none;
		touch-action: manipulation;
	}

	.example_button {
		width: 30%;
		margin-left: 1%;
		margin-right: 1%;
		font-size: 0.9rem;
		background-color: #171717;
	}

	.filter_button:hover,
	.filter_button:focus {
		opacity: 0.75;
	}

	#tableInfo {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		margin-top: 10px;
		margin-bottom: 10px;
	}

	.tableSizeInfo {
		margin-left: auto;
		text-align: right;
	}

	@media (min-width: 768px) {
		.form__field {
			/* width: 100%; */
		}

		.filter_button {
			margin-top: 10px;
			/* width: 100%; */
		}
	}

	@media (max-width: 768px) {
		.form__field {
			width: 100%;
		}

		.filter_button {
			margin-top: 10px;
			width: 100%;
		}
	}
</style>
