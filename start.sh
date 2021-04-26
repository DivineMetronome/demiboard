#!/bin/bash
fuser -k -n tcp 8080 3000
(cd front && npm run dev) & (cd back && cargo-watch -x "run")