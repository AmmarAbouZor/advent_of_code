#! /bin/bash

if [ $# -eq 0 ]; then
	nvim $(rake scaff)
elif [ $1 == "no_test" ]; then
	nvim $(rake scaff_no_test)
else
	echo "Usage: $0 [OPTIONS]"
	echo "Options:"
	echo "  no_test  Specify to not create a test file"
fi
