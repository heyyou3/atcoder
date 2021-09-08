PHONY:	run test submit
SHELL=/bin/bash

q=a

run:
	@go $(q)/main.go

INPUTS = ./${q}/tests/sample-*.in

submit:
	cd ./${q} && \
	acc submit && \
	cd -
