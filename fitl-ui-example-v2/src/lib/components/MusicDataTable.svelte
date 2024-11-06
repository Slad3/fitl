<script>
	import { onMount } from 'svelte';

	export let data;

	$: columns = data.length > 0 ? Object.keys(data[0]) : [];

	$: if (data) {
		console.log('Data has changed:', data);
	}

	onMount(() => {

	});
</script>

<table class="table">
	<thead>
		<tr>
			<th>Artist</th>
			<th>Title</th>
			<th>Album</th>
		</tr>
	</thead>
	<tbody>
		{#each data as song}
			<tr>
				<td>{song.artist}</td>
				<td>{song.title}</td>
				<td>{song.album}</td>
			</tr>
		{/each}
	</tbody>
</table>

<style>
	table {
		width: 100%;
		border-collapse: collapse;
		overflow: hidden;
		box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
		font-size: 0.85rem;
	}

	th,
	td {
		padding: 15px;
		background-color: rgba(255, 255, 255, 0.2);
		color: #fff;
	}

	th {
		text-align: center;
	}

	tbody > tr {
		&:hover {
			background-color: rgba(255, 255, 255, 0.3);
		}
	}

	@media (max-width: 768px) {
		table {
			font-size: 0.7rem;
		}

		th,
		td {
			padding: 10px;
			background-color: rgba(255, 255, 255, 0.2);
			color: #fff;
		}
	}
</style>
