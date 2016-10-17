document.getElementById('submit').addEventListener('click', function(event) {
  event.preventDefault();
})

function apiRequest(method, endpoint) {
  var elements = document.getElementsByTagName('input');
  var dataArray = [];
  var dataObj = {};

  Array.prototype.map.call(elements, function(elem) {
    Object.defineProperty(dataObj, elem.name, {
      enumerable: true,
      configurable: true,
      writable: true,
      value: elem.value
    });
    elem.value = '';
  });

  var body = Object.keys(dataObj).map(function(key) {
    return key + "=" + dataObj[key];
  }).join('&');

  if(method == "GET") {
    body = null;
  }

  return fetch(endpoint, {
    method: method,
    body: body,
    headers: {
      'Content-Type':'application/x-www-form-urlencoded'
    }
  })
  .then(checkStatus)
  .then(function(res) {
    appendData(res);
  })
}

function checkStatus(response) {
  if(response.status >= 200 && response.status < 300) {
    return response.json();
  } else {
    return response.json()
      .then(function(err) {
        console.log(err);
        throw err;
      });
  }
}


function appendData(data) {
   var table_body = document.getElementById('tbody');

   var data_array = Array.prototype.slice.call(table_body.childNodes);
   data_array.map(function(node) {
     table_body.removeChild(node);
   });

   data.map(function(datum) {
     var tr = document.createElement('tr');
     table_body.appendChild(tr);
     Object.keys(datum).map(function(key) {
       if(key == 'purchase_date') {
         datum[key] = new Date(datum[key]).toDateString();
       }
       var td = document.createElement('td');
       td.textContent = datum[key];
       tr.appendChild(td);
     });
   });
}

apiRequest('GET', '/herd');
