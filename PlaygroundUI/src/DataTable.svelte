<script>
  export let data;
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
  /* tbody > td {
    position: relative;
    &:hover {
      &:before {
        content: "";
        position: absolute;
        left: 0;
        right: 0;
        top: -9999px;
        bottom: -9999px;
        background-color: rgba(255, 255, 255, 0.2);
        z-index: -1;
      }
    }
  } */

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
