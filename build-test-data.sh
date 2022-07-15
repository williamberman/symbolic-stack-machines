#! /bin/bash

set -e

for f in test-data/*.sol
do 
    solc --bin-runtime -o test-data --overwrite "$f"
done

for f in test-data/*.yul
do 
    # TODO this is not working
    solc -o test-data --overwrite --strict-assembly "$f"
done
