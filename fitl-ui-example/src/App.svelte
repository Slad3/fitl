<script lang="ts">
  import "./app.css";
  import { onMount } from "svelte";

  import DataTable from "./DataTable.svelte";
  import { example_table } from "./test_data.js";
  import { cleanMap, mapToObj, printError } from "./utils.js";

  import init, { fitl_filter, check_syntax } from "fitl-wasm";

  let queryTextBox;

  let query = "";
  let result_table = [];

  onMount(async () => {
    await init();
    result_table = example_table;
    queryTextBox = document.getElementById("query");
  });

  function colorQueryBox(input) {
    if (!queryTextBox) {
      return;
    }
    queryTextBox.style.borderColor = input ? "green" : "red";
  }

  function submit() {
    try {
      result_table = cleanMap(fitl_filter(query, example_table, "JSARRAY"));
    } catch (error) {
      if (error.name === "RuntimeError") {
        console.error("Runtime Error");
        result_table = example_table;
      } else {
        result_table = [];
        printError(mapToObj(error));
      }
    }
  }

  function onQueryChange(event) {
    query = event.target.value;
    try {
      let temp = check_syntax(query, ["artist", "album", "title"]);
      colorQueryBox(true);
      submit();
    } catch (error) {
      printError(mapToObj(error));

      colorQueryBox(false);
    }
  }

  function example1() {
    query = "artist =: pac";
    onQueryChange({ target: { value: query } });
  }

  function example2() {
    query = "album =: the";
    onQueryChange({ target: { value: query } });
  }

  function example3() {
    query = "artist = 2Pac | artist =: Makaveli";
    onQueryChange({ target: { value: query } });
  }
</script>

<main>
  <h1>FiTL Example</h1>
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
    <button id="submitButton" class="filter_button" on:click={submit}>Filter</button>
  </span>
  <br />
  <br />
  <button class="filter_button example_button" on:click={example1}>All Pac Songs</button>
  <button class="filter_button example_button" on:click={example2}>All "the" Albums</button>
  <button class="filter_button example_button" on:click={example3}>All Pac Songs</button>
  <br />
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
    width: 80%;
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
    background-color: #222;
    border-radius: 4px;
    border-style: none;
    box-sizing: border-box;
    color: #fff;
    cursor: pointer;
    display: inline-block;
    font-family: "Farfetch Basis", "Helvetica Neue", Arial, sans-serif;
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
  }

  .filter_button:hover,
  .filter_button:focus {
    opacity: 0.75;
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
