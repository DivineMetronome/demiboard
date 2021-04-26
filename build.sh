#!/bin/bash
(cd front && npm run build && rm -rf ../static/* && mv dist/* ../static)