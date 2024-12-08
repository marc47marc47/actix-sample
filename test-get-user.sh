#!/bin/sh
id=${1-1}
curl -X GET http://127.0.0.1:8080/user/${id}
