PHONY:	build run
SHELL=/bin/bash

Q=a
OUT_FILE=main.out

build:
	@g++ -g ${Q}/main.cpp -o ${Q}/${OUT_FILE}

run: build
	./${Q}/${OUT_FILE}

INPUTS = ./${Q}/tests/sample-*.in
test: build
	@count=0; for input in ${INPUTS}; do\
		((count = count + 1));\
		echo "Run sample-$$count.in";\
		bash -c "diff <(./${Q}/${OUT_FILE} < "./${Q}/tests/sample-$$count.in") <(cat "./${Q}/tests/sample-$$count.out")";\
	done

submit:
	cd ./${Q} && \
	acc submit && \
	cd -
