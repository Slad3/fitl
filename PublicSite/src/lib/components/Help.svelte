<article class="prose-gray prose-base text-white">
	<h1>Quick guide to writing FiTL queries</h1>

	<h2>Simple Queries</h2>

	<p>
		At it's very root, FiTL filters down rows based on columns and conditional statements for
		described columns.
	</p>
	<p>Base form of a query can be described as:</p>
	<code> *column name* *operatorion* *comparison value* </code>
	<p>For example</p>
	<code>artist = JDilla</code>
	<p>
		Which in a playlist, returns a table where in all the rows the "artist" column is equal to
		"JDilla"
	</p>

	<p>You can inverse this statement by an "!", like:</p>
	<code>artist != JDilla</code>
	<p>or:</p>
	<code>! artist = JDilla</code>

	<p>
		Which filters out all the songs from a playlist where the "artist" column is equal to JDilla
		(idk why anyone would want to do that)
	</p>

	<h3>Other common comparison values</h3>

	<p>
		Naturally comparisons are case case-insensitive. To make them case sensitive, add a "^" in front
		of the operation (but not before the negate value "!")
	</p>

	<code>artist ^= JDilla</code>

	<p>For a more rough estimate in string matching, you can do one of either two operations.</p>

	<p>
		The one that you'll probably be using the most often is the "contains" operation: "=:" which
		instead of checking for exact matches like the "=" operation, checks to see if the column in
		that specific row contains the comparitive value like such:
	</p>

	<code>artist =: dilla</code>

	<p>Tip: the colon is on the side of the smaller side</p>

	<p>
		The inverse of this is the "isin" operation: ":=". This checks to see if the comparison value
		contains the column value like such:
	</p>
	<code>artist := "JDilla and Slumvillage"</code>

	<h2>Combining Queries together</h2>
	<p>You can combine multiple queries together with either a "&" (and) or an "|" (or) symble</p>

	<p>
		The "&" symbole is an "and" operation, meaning that both statements on each side of the "&" must
		be true in order for the query to pass for that row. For example:
	</p>

	<code>artist = "Michael Giacchino" & album =: soundtrack</code>

	<p>
		The above statement only returns rows where the artist name is "Michael Giacchino" AND the album
		title contains the word "soundtrack" in it.
	</p>

	<p>
		The "|" symbol is an "or" operation, meaning only one statement on each side of the "|" symbol
		have to be true in order for the query to pass for that row. For example:
	</p>

	<code>artist = "black thought" | artist =: roots</code>
	<p>
		The above statement returns rows where the artist is equal to "black thought" or the artist
		contains the string "roots" so we can filter through a playlist and get all of Black Thoughts
		solo catalog and also his catalog with The Roots.
	</p>

	<p>
		If you're really advanced (or you don't touch grass), you can get really specific and add
		parenthesis to your queries, which work exactly like they do in math
	</p>

	<code>(artist = "black thought" & album = "Streams of Thought Vol. 1") | artist =: roots</code>

	<p>
		This query returns any Roots song, or if it's a Black Thought song the album is only "Streams of
		Thought Vol. 1".
	</p>

	<p>
		Add as many and nest as many parenthesis as you want. Get crazy with it, I won't care but the
		compiler might yell at you.
	</p>

	<pre><code
			>{'((((((((((artist = "black thought" & album = "Streams of Thought Vol. 1")))))) | (((artist =: roots)))))'}</code
		></pre>

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
		width: 80%;
	}

	code {
		background-color: black;
		padding: 10px;
		padding-left: 20px;
		padding-right: 20px;
		border-radius: 7px;
		color: whitesmoke;
	}

	h2,
	h3 {
		text-align: center;
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
