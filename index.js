


const express = require('express')
const app = express()
const port = 31430

app.use(express.json());

app.post('/ocr_webhook', function(request, response){
  console.log(request.body);      // your JSON
   response.send(request.body);    // echo the result back
});
console.log('start')
app.listen(port);