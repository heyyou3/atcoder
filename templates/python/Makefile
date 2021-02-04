PHONY:	run test submit
SHELL=/bin/bash

Q=a

run:
	@python3 $(Q)/main.py

INPUTS = ./${Q}/tests/sample-*.in
test:
	@count=0; for input in ${INPUTS}; do\
		((count = count + 1));\
		echo "Run sample-$$count.in";\
		bash -c "diff <(python3 $(Q)/main.py < "./${Q}/tests/sample-$$count.in") <(cat "./${Q}/tests/sample-$$count.out")";\
	done

submit: test
	cd ./${Q} && \
	acc submit && \
	cd -
