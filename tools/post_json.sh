#/bin/bash
curl -H "Content-Type: application/json" -X POST -d '{"map":{"width":7,"height":5,"start":{"x":1,"y":1},"destination":{"x":4,"y":1},"fields":[1,1,1,1,1,1,1,1,0.1,0.7,0.1,0.1,0.1,1,1,0.1,0.1,0.1,0.1,0.1,1,1,0.1,0.1,0.1,0.1,0.1,1,1,1,1,1,1,1,1]}}' http://localhost:3000/
