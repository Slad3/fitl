<script lang="ts">
	//@ts-ignore
	import { capitalize, sort_columns } from '$lib/utils';

	export let data: Array<{ [key: string]: any }>;
	export let order: string[] = [];

	$: columns = sort_columns(data, order);
</script>

<div class="relative overflow-x-auto shadow-md sm:rounded-lg">
	<table class="w-full text-sm text-left rtl:text-right text-gray-400">
		<thead class="text-xs uppercase bg-table-header text-white">
			<tr>
				{#each columns as column}
					<th scope="col" class="px-6 py-3">{capitalize(column)}</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each data as row}
				<tr class=" odd:bg-table-primary even:bg-table-secondary border-b border-table-header">
					{#each columns as column}
						<th
							scope="row"
							class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
							>{row[column]}</th
						>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	table {
		width: 100%;
		border-collapse: collapse;
		overflow: hidden;
		box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
		font-size: 0.85rem;
	}
</style>
