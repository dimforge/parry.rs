#!/bin/bash
set -v

if gsed --help >  /dev/null 2>&1; then
    gsed=gsed
    echo "gsed found: using gsed."
else
    # Hopefully installed sed is the gnu one.
    # note: aliases work poorly with xargs.
    gsed=sed
    echo "gsed not found: using sed."
fi

mkdir -p docs/user_guide/rust/

find docs/user_guide/templates/ -type f -print0 | OUTPUT_FOLDER=docs/user_guide xargs -0 ./inject_code_in_user_guide.sh
