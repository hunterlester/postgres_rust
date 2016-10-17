<html>
  <head>
    <meta charset="utf-8">
    <title>Herd Inventory</title>
  </head>
  <body>
    <h4>This client is being served by a web server written in Rust language. <a target="_blank" href="https://www.rust-lang.org">rust-lang.org</a> This is a project to help me, <a target="_blank" href="https://github.com/hunterlester/postgres_rust">https://github.com/hunterlester/postgres_rust</a>, to learn to program in Rust and to learn the PostgresQL database.</h4>
    <form id="herd_form">
      <label for="breed">Breed</label>
      <input type="text" name="breed">

      <label for="name">Name</label>
      <input type="text" name="name">

      <label for="purchase_date">Purchase Date</label>
      <input type="date" name="purchase_date">

      <button type="submit" id="submit" onclick="apiRequest('POST', '/herd')">Submit</button>
    </form>
    <br>
    <table>
      <thead>
        <tr>
          <th>
            Tag ID
          </th>
          <th>
            Breed
          </th>
          <th>
            Name
          </th>
          <th>
            Purchase Date
          </th>
        </tr>
      </thead>
      <tbody id="tbody">

      </tbody>
    </table>

    <script type="text/javascript" src="clientFetch.js">

    </script>
  </body>
</html>
