#/bin/bash
curl -H "Content-Type: application/json" -X POST -d '{"width":7,"height":5,"field":[1,1,1,1,1,1,1,0.1,0.7,0.1,0.1,0.1,1,1,0.1,0.1,0.1,0.1,0.1,1,1,0.1,0.1,0.1,0.1,0.1,1,1,1,1,1,1,1,1],"start":[1,1],"destination":[4,1]}' http://localhost:3000/
