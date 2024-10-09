<script>
  import "./app.css";
  import { onMount } from "svelte";
  //   import init, { test_json_input, fitl_filter } from "../public/buildasdfasdf/fitl_wasm.js"; // Working wasm solution

  import DataTable from "./DataTable.svelte";

  import init, { fitl_filter } from "fitl-wasm";

  let example_table = [
    { artist: "2Pac", album: "Me Against the World", title: "So Many Tears" },
    { artist: "2Pac", album: "Me Against the World", title: "Lord Knows" },
    { artist: "2Pac", album: "All Eyez on Me", title: "All Eyez on Me" },
    { artist: "2Pac", album: "All Eyez on Me", title: "2 Of Amerikaz Most Wanted" },
    { artist: "2Pac", album: "All Eyez on Me", title: "Heartz of Men" },
    { artist: "Makaveli", album: "The Don Killuminati: The 7 Day Theory", title: "Toss It Up" },
    {
      artist: "Makaveli",
      album: "The Don Killuminati: The 7 Day Theory",
      title: "Me And My Girlfriend",
    },
    { artist: "Makaveli", album: "The Don Killuminati: The 7 Day Theory", title: "Against All Odds" },
  ];

  let query = "";
  let result_table = [];

  onMount(async () => {
    await init();
    console.log(example_table);
    result_table = example_table;
  });

  function onQueryChange(event) {
    query = event.target.value;
    try {
      result_table = fitl_filter(query, example_table, "JSARRAY").map((map) => Object.fromEntries(map));
      console.log(result_table.length);
    } catch (error) {
      result_table = [];
      console.error(error);
    }
  }
</script>

<main>
  <h1>FiTL Example</h1>
  <div class="form__group field">
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
  </div>
  <br />

  <DataTable data={result_table}></DataTable>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 800px;
    margin: 0 auto;
  }

  :root {
    --primary: #9b9b9b;
    /* --secondary: #38ef7d; */
    --secondary: black;
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
    top: 20px;
  }

  .form__label {
    position: absolute;
    top: 0;
    display: block;
    transition: 0.2s;
    font-size: 1rem;
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
    border-image: linear-gradient(to right, var(--primary), var(--secondary));
    border-image-slice: 1;
  }

  .form__field:required,
  .form__field:invalid {
    box-shadow: none;
  }

  /* demo */
  body {
    font-family: "Poppins", sans-serif;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    font-size: 1.5rem;
    background-color: #222222;
  }
</style>
