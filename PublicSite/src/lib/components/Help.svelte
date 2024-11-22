<article class="prose-gray prose-base text-white">
	<h1>Writing Queries</h1>

	<h2>Simple Queries</h2>
	<p>Base form of a query can be described as:</p>
	<code> *column* *operator* *value* </code>
	<p>For example</p>
	<code>artist = JDilla</code>
	<p>Which filters through all the rows where the "artist" is equal to "JDilla"</p>
	<h2>Get Complex With It</h2>
	<p>You can add multiple queries together with either a "&" (and) or an "|" (or) symble</p>
	<code>artist =: pac | artist = Makaveli</code>
	<p>Returns a table where the artist either has "pac" in the string or is equal to "Makaveli"</p>

	<h1>Symbol Table</h1>
	<table>
		<thead>
			<tr>
				<th>Word</th>
				<th>Symbol</th>
				<th>Description</th>
			</tr>
		</thead>
		<tbody>
			<tr>
				<td>not</td>
				<td>!T</td>
				<td>Negates Operation</td>
			</tr>
			<tr>
				<td>is / equals</td>
				<td>=</td>
				<td>Exact match</td>
			</tr>
			<tr>
				<td>contains</td>
				<td>=:</td>
				<td>Left contains right (Nickolas Picklous =: Nick)</td>
			</tr>
			<tr>
				<td>isin</td>
				<td>:=</td>
				<td>Right contains left (Nick := Nickolas Picklous)</td>
			</tr>
			<tr>
				<td>lessthan</td>
				<td>&lt;</td>
				<td>"Less than" comparison of numbers or characters based on ASCII value of characters</td>
			</tr>
			<tr>
				<td>morethan</td>
				<td>&gt;</td>
				<td
					>"Greater than" comparison of numbers or characters based on ASCII value of characters</td
				>
			</tr>
			<tr>
				<td>lessthanequals</td>
				<td>&lt;=</td>
				<td
					>"Less than or equals" comparison of numbers or characters based on ASCII value of
					characters</td
				>
			</tr>
			<tr>
				<td>morethanequals</td>
				<td>&gt;=</td>
				<td
					>"Greater than or equals" comparison of numbers or characters based on ASCII value of
					characters</td
				>
			</tr>
			<tr>
				<td>or</td>
				<td>|</td>
				<td>Or boolean operation</td>
			</tr>
			<tr>
				<td>and</td>
				<td>&amp;</td>
				<td>And boolean operation</td>
			</tr>
			<tr>
				<td><em>Parenthesis</em></td>
				<td>()</td>
				<td>Prioritizes statements inside parenthesis</td>
			</tr>
			<tr>
				<td><em>NA</em></td>
				<td>"&lt;value&gt;"</td>
				<td
					>Combines multiple words into single string. Necessary for multi-worded tokens, optional
					for single-worded tokens</td
				>
			</tr>
			<tr>
				<td><em>NA</em></td>
				<td>^T</td>
				<td>Makes statement case-sensitive; queries are case-insensitive by default</td>
			</tr>
		</tbody>
	</table>
</article>

<style>
	article {
		margin-left: 10%;
		width: 80%;
		margin-right: 10%;
	}

	code {
		background-color: black;
		padding: 10px;
		padding-left: 20px;
		padding-right: 20px;
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
