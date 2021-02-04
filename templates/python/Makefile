PHONY:	run test submit
SHELL=/bin/bash

q=a

run:
	@python3 $(q)/main.py

INPUTS = ./${q}/tests/sample-*.in
test:
	@count=0; for input in ${INPUTS}; do\
		((count = count + 1));\
		echo "Run sample-$$count.in";\
		bash -c "diff <(python3 $(q)/main.py < "./${q}/tests/sample-$$count.in") <(cat "./${q}/tests/sample-$$count.out")";\
	done

submit: test
	cd ./${q} && \
	acc submit && \
	cd -
