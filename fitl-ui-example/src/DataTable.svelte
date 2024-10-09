<script>
  export let data;
</script>

<table class="table">
  <thead>
    <tr>
      <th>{data.length}</th>
      <th>Artist</th>
      <th>Album</th>
      <th>Title</th>
    </tr>
  </thead>
  <tbody>
    {#each data as song}
      <tr>
        <td></td>
        <td>{song.artist}</td>
        <td>{song.title}</td>
        <td>{song.album}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .container {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  table {
    width: 800px;
    border-collapse: collapse;
    overflow: hidden;
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
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

  thead > th {
    background-color: #55608f;
  }

  tbody > tr {
    &:hover {
      background-color: rgba(255, 255, 255, 0.3);
    }
  }
  tbody > td {
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
  }
</style>
